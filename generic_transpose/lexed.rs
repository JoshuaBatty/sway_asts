Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0ef5d6490,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0ef5d6490,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Struct(
                            ItemStruct {
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 29,
                                        as_str(): "GenericStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 29,
                                                    end: 30,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 30,
                                                            end: 31,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 31,
                                                    end: 32,
                                                    as_str(): ">",
                                                },
                                            },
                                        },
                                    },
                                ),
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
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 39,
                                                            end: 40,
                                                            as_str(): "x",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 40,
                                                            end: 41,
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
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 41,
                                                                        end: 42,
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
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 33,
                                        end: 44,
                                        as_str(): "{\n    x:T\n}",
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
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 46,
                                        end: 50,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 50,
                                                    end: 51,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 51,
                                                                end: 52,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 52,
                                                                end: 53,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 54,
                                                            end: 55,
                                                            as_str(): "E",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 55,
                                                    end: 56,
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
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 57,
                                                    end: 63,
                                                    as_str(): "Option",
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
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 63,
                                                                    end: 64,
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
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 64,
                                                                                        end: 70,
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
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 70,
                                                                                                        end: 71,
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
                                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 71,
                                                                                                                                end: 72,
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
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 72,
                                                                                                                    end: 73,
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
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 74,
                                                                                                                            end: 75,
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
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 75,
                                                                                                        end: 76,
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
                                                            },
                                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 76,
                                                                    end: 77,
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
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 84,
                                                                end: 87,
                                                                as_str(): "pub",
                                                            },
                                                        },
                                                    ),
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 88,
                                                            end: 90,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 91,
                                                            end: 100,
                                                            as_str(): "transpose",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 101,
                                                                    end: 105,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 100,
                                                            end: 106,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 107,
                                                                    end: 109,
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
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 110,
                                                                                end: 116,
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
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 116,
                                                                                                end: 117,
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
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 117,
                                                                                                                        end: 123,
                                                                                                                        as_str(): "Option",
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
                                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 123,
                                                                                                                                        end: 124,
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
                                                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 124,
                                                                                                                                                            end: 125,
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
                                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 125,
                                                                                                                                        end: 126,
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
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 126,
                                                                                                            end: 127,
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
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 128,
                                                                                                                    end: 129,
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
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 129,
                                                                                                end: 130,
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
                                                        statements: [],
                                                        final_expr_opt: Some(
                                                            Match {
                                                                match_token: MatchToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 139,
                                                                        end: 144,
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
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 145,
                                                                                    end: 149,
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
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 162,
                                                                                                end: 168,
                                                                                                as_str(): "Option",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 168,
                                                                                                    end: 170,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 170,
                                                                                                        end: 174,
                                                                                                        as_str(): "Some",
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
                                                                                            Constructor {
                                                                                                path: PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 175,
                                                                                                                end: 181,
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
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 181,
                                                                                                                    end: 183,
                                                                                                                    as_str(): "::",
                                                                                                                },
                                                                                                            },
                                                                                                            PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 183,
                                                                                                                        end: 185,
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
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 186,
                                                                                                                        end: 187,
                                                                                                                        as_str(): "x",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 185,
                                                                                                        end: 188,
                                                                                                        as_str(): "(x)",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 174,
                                                                                        end: 189,
                                                                                        as_str(): "(Result::Ok(x))",
                                                                                    },
                                                                                },
                                                                            },
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 190,
                                                                                    end: 192,
                                                                                    as_str(): "=>",
                                                                                },
                                                                            },
                                                                            kind: Expr {
                                                                                expr: FuncApp {
                                                                                    func: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 193,
                                                                                                        end: 199,
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
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 199,
                                                                                                            end: 201,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 201,
                                                                                                                end: 203,
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
                                                                                                FuncApp {
                                                                                                    func: Path(
                                                                                                        PathExpr {
                                                                                                            root_opt: None,
                                                                                                            prefix: PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 204,
                                                                                                                        end: 210,
                                                                                                                        as_str(): "Option",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                generics_opt: None,
                                                                                                            },
                                                                                                            suffix: [
                                                                                                                (
                                                                                                                    DoubleColonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 210,
                                                                                                                            end: 212,
                                                                                                                            as_str(): "::",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 212,
                                                                                                                                end: 216,
                                                                                                                                as_str(): "Some",
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
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 217,
                                                                                                                                    end: 218,
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
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 216,
                                                                                                            end: 219,
                                                                                                            as_str(): "(x)",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 203,
                                                                                            end: 220,
                                                                                            as_str(): "(Option::Some(x))",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                comma_token: CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 220,
                                                                                        end: 221,
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
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 232,
                                                                                                end: 238,
                                                                                                as_str(): "Option",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 238,
                                                                                                    end: 240,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 240,
                                                                                                        end: 244,
                                                                                                        as_str(): "Some",
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
                                                                                            Constructor {
                                                                                                path: PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 245,
                                                                                                                end: 251,
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
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 251,
                                                                                                                    end: 253,
                                                                                                                    as_str(): "::",
                                                                                                                },
                                                                                                            },
                                                                                                            PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 253,
                                                                                                                        end: 256,
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
                                                                                                            Var {
                                                                                                                reference: None,
                                                                                                                mutable: None,
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 257,
                                                                                                                        end: 258,
                                                                                                                        as_str(): "e",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 256,
                                                                                                        end: 259,
                                                                                                        as_str(): "(e)",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 244,
                                                                                        end: 260,
                                                                                        as_str(): "(Result::Err(e))",
                                                                                    },
                                                                                },
                                                                            },
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 261,
                                                                                    end: 263,
                                                                                    as_str(): "=>",
                                                                                },
                                                                            },
                                                                            kind: Expr {
                                                                                expr: FuncApp {
                                                                                    func: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 264,
                                                                                                        end: 270,
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
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 270,
                                                                                                            end: 272,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 272,
                                                                                                                end: 275,
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
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 276,
                                                                                                                    end: 277,
                                                                                                                    as_str(): "e",
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
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 275,
                                                                                            end: 278,
                                                                                            as_str(): "(e)",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                comma_token: CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 278,
                                                                                        end: 279,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                        MatchBranch {
                                                                            pattern: Constant(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 290,
                                                                                                end: 296,
                                                                                                as_str(): "Option",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 296,
                                                                                                    end: 298,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 298,
                                                                                                        end: 302,
                                                                                                        as_str(): "None",
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
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 303,
                                                                                    end: 305,
                                                                                    as_str(): "=>",
                                                                                },
                                                                            },
                                                                            kind: Expr {
                                                                                expr: FuncApp {
                                                                                    func: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 306,
                                                                                                        end: 312,
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
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 312,
                                                                                                            end: 314,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 314,
                                                                                                                end: 316,
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
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 317,
                                                                                                                    end: 323,
                                                                                                                    as_str(): "Option",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            generics_opt: None,
                                                                                                        },
                                                                                                        suffix: [
                                                                                                            (
                                                                                                                DoubleColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 323,
                                                                                                                        end: 325,
                                                                                                                        as_str(): "::",
                                                                                                                    },
                                                                                                                },
                                                                                                                PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 325,
                                                                                                                            end: 329,
                                                                                                                            as_str(): "None",
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
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 316,
                                                                                            end: 330,
                                                                                            as_str(): "(Option::None)",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                comma_token: CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 330,
                                                                                        end: 331,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 150,
                                                                        end: 339,
                                                                        as_str(): "{\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 131,
                                                        end: 345,
                                                        as_str(): "{\n      match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 78,
                                        end: 347,
                                        as_str(): "{\n    pub fn transpose(self) -> Result<Option<T>, E> {\n      match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }\n    }\n}",
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
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 349,
                                        end: 353,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 353,
                                                    end: 354,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 354,
                                                            end: 355,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 355,
                                                    end: 356,
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
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 357,
                                                    end: 370,
                                                    as_str(): "GenericStruct",
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
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 370,
                                                                    end: 371,
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
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 371,
                                                                                        end: 377,
                                                                                        as_str(): "Option",
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
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 377,
                                                                                                        end: 378,
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
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 378,
                                                                                                                            end: 379,
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
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 379,
                                                                                                        end: 380,
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
                                                            },
                                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 380,
                                                                    end: 381,
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
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 388,
                                                                end: 391,
                                                                as_str(): "pub",
                                                            },
                                                        },
                                                    ),
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 392,
                                                            end: 394,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 395,
                                                            end: 404,
                                                            as_str(): "transpose",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 405,
                                                                    end: 409,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 404,
                                                            end: 410,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 411,
                                                                    end: 413,
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
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 414,
                                                                                end: 420,
                                                                                as_str(): "Option",
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
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 420,
                                                                                                end: 421,
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
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 421,
                                                                                                                    end: 434,
                                                                                                                    as_str(): "GenericStruct",
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
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 434,
                                                                                                                                    end: 435,
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
                                                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 435,
                                                                                                                                                        end: 436,
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
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 436,
                                                                                                                                    end: 437,
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
                                                                                        },
                                                                                        close_angle_bracket_token: CloseAngleBracketToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 437,
                                                                                                end: 438,
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
                                                        statements: [],
                                                        final_expr_opt: Some(
                                                            Match {
                                                                match_token: MatchToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 447,
                                                                        end: 452,
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
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 453,
                                                                                    end: 457,
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
                                                                            pattern: Struct {
                                                                                path: PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 470,
                                                                                                end: 483,
                                                                                                as_str(): "GenericStruct",
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
                                                                                            Field {
                                                                                                field_name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 484,
                                                                                                        end: 485,
                                                                                                        as_str(): "x",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                pattern_opt: Some(
                                                                                                    (
                                                                                                        ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 485,
                                                                                                                end: 486,
                                                                                                                as_str(): ":",
                                                                                                            },
                                                                                                        },
                                                                                                        Constructor {
                                                                                                            path: PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 486,
                                                                                                                            end: 492,
                                                                                                                            as_str(): "Option",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    generics_opt: None,
                                                                                                                },
                                                                                                                suffix: [
                                                                                                                    (
                                                                                                                        DoubleColonToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 492,
                                                                                                                                end: 494,
                                                                                                                                as_str(): "::",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        PathExprSegment {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 494,
                                                                                                                                    end: 498,
                                                                                                                                    as_str(): "Some",
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
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 499,
                                                                                                                                    end: 500,
                                                                                                                                    as_str(): "y",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 498,
                                                                                                                    end: 501,
                                                                                                                    as_str(): "(y)",
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 483,
                                                                                        end: 502,
                                                                                        as_str(): "{x:Option::Some(y)}",
                                                                                    },
                                                                                },
                                                                            },
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 503,
                                                                                    end: 505,
                                                                                    as_str(): "=>",
                                                                                },
                                                                            },
                                                                            kind: Expr {
                                                                                expr: FuncApp {
                                                                                    func: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 506,
                                                                                                        end: 512,
                                                                                                        as_str(): "Option",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 512,
                                                                                                            end: 514,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 514,
                                                                                                                end: 518,
                                                                                                                as_str(): "Some",
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
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 519,
                                                                                                                    end: 532,
                                                                                                                    as_str(): "GenericStruct",
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
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 534,
                                                                                                                            end: 535,
                                                                                                                            as_str(): "x",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    expr_opt: Some(
                                                                                                                        (
                                                                                                                            ColonToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 535,
                                                                                                                                    end: 536,
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
                                                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 537,
                                                                                                                                                end: 538,
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
                                                                                                                    ),
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 532,
                                                                                                            end: 539,
                                                                                                            as_str(): "{ x: y}",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 518,
                                                                                            end: 540,
                                                                                            as_str(): "(GenericStruct{ x: y})",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                comma_token: CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 540,
                                                                                        end: 541,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                        MatchBranch {
                                                                            pattern: Struct {
                                                                                path: PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 552,
                                                                                                end: 565,
                                                                                                as_str(): "GenericStruct",
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
                                                                                            Field {
                                                                                                field_name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 566,
                                                                                                        end: 567,
                                                                                                        as_str(): "x",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                pattern_opt: Some(
                                                                                                    (
                                                                                                        ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 567,
                                                                                                                end: 568,
                                                                                                                as_str(): ":",
                                                                                                            },
                                                                                                        },
                                                                                                        Constant(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 568,
                                                                                                                            end: 574,
                                                                                                                            as_str(): "Option",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    generics_opt: None,
                                                                                                                },
                                                                                                                suffix: [
                                                                                                                    (
                                                                                                                        DoubleColonToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 574,
                                                                                                                                end: 576,
                                                                                                                                as_str(): "::",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        PathExprSegment {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 576,
                                                                                                                                    end: 580,
                                                                                                                                    as_str(): "None",
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
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 565,
                                                                                        end: 581,
                                                                                        as_str(): "{x:Option::None}",
                                                                                    },
                                                                                },
                                                                            },
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 582,
                                                                                    end: 584,
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
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 585,
                                                                                                    end: 591,
                                                                                                    as_str(): "Option",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                        suffix: [
                                                                                            (
                                                                                                DoubleColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 591,
                                                                                                        end: 593,
                                                                                                        as_str(): "::",
                                                                                                    },
                                                                                                },
                                                                                                PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 593,
                                                                                                            end: 597,
                                                                                                            as_str(): "None",
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
                                                                                comma_token: CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 597,
                                                                                        end: 598,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 458,
                                                                        end: 606,
                                                                        as_str(): "{\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 439,
                                                        end: 612,
                                                        as_str(): "{\n      match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 382,
                                        end: 614,
                                        as_str(): "{\n    pub fn transpose(self) -> Option<GenericStruct<T>> {\n      match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }\n    }\n}",
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
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 616,
                                            end: 618,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 619,
                                            end: 623,
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
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 623,
                                            end: 625,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 626,
                                                    end: 628,
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
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 629,
                                                                end: 633,
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
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 640,
                                                            end: 643,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 644,
                                                                end: 645,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 645,
                                                                    end: 646,
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
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 647,
                                                                                end: 653,
                                                                                as_str(): "Option",
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
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 653,
                                                                                                end: 654,
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
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 654,
                                                                                                                    end: 660,
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
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 660,
                                                                                                                                    end: 661,
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
                                                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 661,
                                                                                                                                                            end: 664,
                                                                                                                                                            as_str(): "u64",
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
                                                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 664,
                                                                                                                                                end: 665,
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
                                                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 666,
                                                                                                                                                        end: 668,
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
                                                                                                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 668,
                                                                                                                                    end: 669,
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
                                                                                        },
                                                                                        close_angle_bracket_token: CloseAngleBracketToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 669,
                                                                                                end: 670,
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 671,
                                                            end: 672,
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
                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                            ),
                                                                            start: 673,
                                                                            end: 679,
                                                                            as_str(): "Option",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 679,
                                                                                end: 681,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 681,
                                                                                    end: 685,
                                                                                    as_str(): "Some",
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
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 686,
                                                                                            end: 692,
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
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 692,
                                                                                                end: 694,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 694,
                                                                                                    end: 696,
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 697,
                                                                                                    end: 698,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 696,
                                                                                end: 699,
                                                                                as_str(): "(5)",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 685,
                                                                end: 700,
                                                                as_str(): "(Result::Ok(5))",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 700,
                                                            end: 701,
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
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 706,
                                                                        end: 712,
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
                                                                        target: MethodCall {
                                                                            target: MethodCall {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 713,
                                                                                                    end: 714,
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
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 714,
                                                                                        end: 715,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                path_seg: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 715,
                                                                                            end: 724,
                                                                                            as_str(): "transpose",
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
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 724,
                                                                                        end: 726,
                                                                                        as_str(): "()",
                                                                                    },
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 726,
                                                                                    end: 727,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            path_seg: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 727,
                                                                                        end: 733,
                                                                                        as_str(): "unwrap",
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
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 733,
                                                                                    end: 735,
                                                                                    as_str(): "()",
                                                                                },
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 735,
                                                                                end: 736,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        path_seg: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 736,
                                                                                    end: 742,
                                                                                    as_str(): "unwrap",
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
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 742,
                                                                                end: 744,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                            ),
                                                                            start: 745,
                                                                            end: 747,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 748,
                                                                                    end: 749,
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
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 712,
                                                            end: 750,
                                                            as_str(): "(y.transpose().unwrap().unwrap() == 5)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 750,
                                                            end: 751,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 757,
                                                            end: 760,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 761,
                                                                end: 762,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 762,
                                                                    end: 763,
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
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 764,
                                                                                end: 777,
                                                                                as_str(): "GenericStruct",
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
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 777,
                                                                                                end: 778,
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
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 778,
                                                                                                                    end: 784,
                                                                                                                    as_str(): "Option",
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
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 784,
                                                                                                                                    end: 785,
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
                                                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 785,
                                                                                                                                                        end: 788,
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
                                                                                                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 788,
                                                                                                                                    end: 789,
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
                                                                                        },
                                                                                        close_angle_bracket_token: CloseAngleBracketToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 789,
                                                                                                end: 790,
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 791,
                                                            end: 792,
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
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 793,
                                                                        end: 806,
                                                                        as_str(): "GenericStruct",
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
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 808,
                                                                                end: 809,
                                                                                as_str(): "x",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 809,
                                                                                        end: 810,
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
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 811,
                                                                                                        end: 817,
                                                                                                        as_str(): "Option",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 817,
                                                                                                            end: 819,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 819,
                                                                                                                end: 823,
                                                                                                                as_str(): "Some",
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
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 824,
                                                                                                                end: 825,
                                                                                                                as_str(): "5",
                                                                                                            },
                                                                                                            parsed: 5,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 823,
                                                                                            end: 826,
                                                                                            as_str(): "(5)",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 806,
                                                                end: 827,
                                                                as_str(): "{ x: Option::Some(5)}",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 827,
                                                            end: 828,
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
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 833,
                                                                        end: 839,
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
                                                                        target: MethodCall {
                                                                            target: MethodCall {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 840,
                                                                                                    end: 841,
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
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 841,
                                                                                        end: 842,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                path_seg: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 842,
                                                                                            end: 851,
                                                                                            as_str(): "transpose",
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
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 851,
                                                                                        end: 853,
                                                                                        as_str(): "()",
                                                                                    },
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 853,
                                                                                    end: 854,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            path_seg: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 854,
                                                                                        end: 860,
                                                                                        as_str(): "unwrap",
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
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 860,
                                                                                    end: 862,
                                                                                    as_str(): "()",
                                                                                },
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 862,
                                                                                end: 863,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 863,
                                                                                end: 864,
                                                                                as_str(): "x",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                            ),
                                                                            start: 865,
                                                                            end: 867,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 868,
                                                                                    end: 869,
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
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 839,
                                                            end: 870,
                                                            as_str(): "(y.transpose().unwrap().x == 5)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 870,
                                                            end: 871,
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
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 877,
                                                            end: 881,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 634,
                                        end: 883,
                                        as_str(): "{\n    let y: Option<Result<u64, u8>> = Option::Some(Result::Ok(5));\n    assert(y.transpose().unwrap().unwrap() == 5);\n\n    let y: GenericStruct<Option<u64>> = GenericStruct{ x: Option::Some(5)};\n    assert(y.transpose().unwrap().x == 5);\n\n    true\n}",
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
