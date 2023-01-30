Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Dependency(
                            Dependency {
                                dep_token: DepToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 12,
                                        as_str(): "dep",
                                    },
                                },
                                path: DependencyPath {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 18,
                                            as_str(): "utils",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 18,
                                        end: 19,
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 21,
                                        end: 24,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 25,
                                            end: 28,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 28,
                                            end: 30,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 30,
                                                end: 36,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 36,
                                                end: 38,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 38,
                                                    end: 44,
                                                    as_str(): "assert",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 44,
                                        end: 45,
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 47,
                                        end: 50,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 51,
                                            end: 56,
                                            as_str(): "utils",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 56,
                                            end: 58,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Glob {
                                        star_token: StarToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 58,
                                                end: 59,
                                                as_str(): "*",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 59,
                                        end: 60,
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 62,
                                        end: 68,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 69,
                                        end: 79,
                                        as_str(): "CustomType",
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 86,
                                                                end: 90,
                                                                as_str(): "name",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 90,
                                                                end: 91,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Str {
                                                            str_token: StrToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 92,
                                                                    end: 95,
                                                                    as_str(): "str",
                                                                },
                                                            },
                                                            length: SquareBrackets {
                                                                inner: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 96,
                                                                                end: 97,
                                                                                as_str(): "3",
                                                                            },
                                                                            parsed: 3,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 95,
                                                                    end: 98,
                                                                    as_str(): "[3]",
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 98,
                                                        end: 99,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 80,
                                        end: 101,
                                        as_str(): "{\n    name: str[3],\n}",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 103,
                                        end: 107,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 108,
                                        end: 116,
                                        as_str(): "MyResult",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 116,
                                                    end: 117,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 117,
                                                                end: 118,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 118,
                                                                end: 119,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 120,
                                                            end: 121,
                                                            as_str(): "E",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 121,
                                                    end: 122,
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 129,
                                                                end: 131,
                                                                as_str(): "Ok",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
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
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 133,
                                                                            end: 134,
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
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 134,
                                                        end: 135,
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 140,
                                                                end: 143,
                                                                as_str(): "Err",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 143,
                                                                end: 144,
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
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 145,
                                                                            end: 146,
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
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 123,
                                        end: 149,
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 151,
                                        end: 157,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 158,
                                        end: 168,
                                        as_str(): "SomeStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 168,
                                                    end: 169,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 170,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 170,
                                                    end: 171,
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 178,
                                                                end: 179,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 179,
                                                                end: 180,
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
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 181,
                                                                            end: 182,
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
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 182,
                                                        end: 183,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 172,
                                        end: 185,
                                        as_str(): "{\n    a: T,\n}",
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
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 187,
                                            end: 189,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 190,
                                            end: 205,
                                            as_str(): "simple_vec_test",
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
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 205,
                                            end: 207,
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 214,
                                                            end: 217,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 218,
                                                                    end: 221,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 222,
                                                                end: 226,
                                                                as_str(): "vec1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 227,
                                                            end: 228,
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
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 229,
                                                                            end: 232,
                                                                            as_str(): "Vec",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 232,
                                                                                end: 234,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 234,
                                                                                    end: 237,
                                                                                    as_str(): "new",
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
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 237,
                                                                end: 239,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 239,
                                                            end: 240,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 245,
                                                            end: 248,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 249,
                                                                    end: 252,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 253,
                                                                end: 257,
                                                                as_str(): "vec2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 258,
                                                            end: 259,
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
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 260,
                                                                            end: 263,
                                                                            as_str(): "Vec",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 263,
                                                                                end: 265,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 265,
                                                                                    end: 268,
                                                                                    as_str(): "new",
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
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 268,
                                                                end: 270,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 270,
                                                            end: 271,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 277,
                                                                        end: 281,
                                                                        as_str(): "vec2",
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 281,
                                                            end: 282,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 282,
                                                                end: 286,
                                                                as_str(): "push",
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
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 287,
                                                                                end: 289,
                                                                                as_str(): "54",
                                                                            },
                                                                            parsed: 54,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U32,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 289,
                                                                                        end: 292,
                                                                                        as_str(): "u32",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 286,
                                                            end: 293,
                                                            as_str(): "(54u32)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 293,
                                                            end: 294,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 299,
                                                                        end: 303,
                                                                        as_str(): "vec1",
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 303,
                                                            end: 304,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 304,
                                                                end: 308,
                                                                as_str(): "push",
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
                                                                Struct {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 309,
                                                                                    end: 319,
                                                                                    as_str(): "SomeStruct",
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
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 322,
                                                                                            end: 323,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    expr_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 323,
                                                                                                    end: 324,
                                                                                                    as_str(): ":",
                                                                                                },
                                                                                            },
                                                                                            Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 325,
                                                                                                            end: 327,
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
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 320,
                                                                            end: 329,
                                                                            as_str(): "{ a: 42 }",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 308,
                                                            end: 330,
                                                            as_str(): "(SomeStruct { a: 42 })",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 330,
                                                            end: 331,
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
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 337,
                                                                        end: 343,
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
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 344,
                                                                                                    end: 348,
                                                                                                    as_str(): "vec1",
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
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 348,
                                                                                        end: 349,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                path_seg: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 349,
                                                                                            end: 352,
                                                                                            as_str(): "get",
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
                                                                                            Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 353,
                                                                                                            end: 354,
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
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 352,
                                                                                        end: 355,
                                                                                        as_str(): "(0)",
                                                                                    },
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 355,
                                                                                    end: 356,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            path_seg: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 356,
                                                                                        end: 362,
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
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 362,
                                                                                    end: 364,
                                                                                    as_str(): "()",
                                                                                },
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 364,
                                                                                end: 365,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 365,
                                                                                end: 366,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 367,
                                                                            end: 369,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 370,
                                                                                    end: 372,
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 343,
                                                            end: 373,
                                                            as_str(): "(vec1.get(0).unwrap().a == 42)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 373,
                                                            end: 374,
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
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 379,
                                                                        end: 385,
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
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 386,
                                                                                                end: 390,
                                                                                                as_str(): "vec2",
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
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 390,
                                                                                    end: 391,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            path_seg: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 391,
                                                                                        end: 394,
                                                                                        as_str(): "get",
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
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 395,
                                                                                                        end: 396,
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
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 394,
                                                                                    end: 397,
                                                                                    as_str(): "(0)",
                                                                                },
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 397,
                                                                                end: 398,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        path_seg: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 398,
                                                                                    end: 404,
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 404,
                                                                                end: 406,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 407,
                                                                            end: 409,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 410,
                                                                                    end: 412,
                                                                                    as_str(): "54",
                                                                                },
                                                                                parsed: 54,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 412,
                                                                                            end: 415,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 385,
                                                            end: 416,
                                                            as_str(): "(vec2.get(0).unwrap() == 54u32)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 416,
                                                            end: 417,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 208,
                                        end: 419,
                                        as_str(): "{\n    let mut vec1 = Vec::new();\n    let mut vec2 = Vec::new();\n\n    vec2.push(54u32);\n    vec1.push(SomeStruct { a: 42 });\n\n    assert(vec1.get(0).unwrap().a == 42);\n    assert(vec2.get(0).unwrap() == 54u32);\n}",
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
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 421,
                                            end: 423,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 424,
                                            end: 440,
                                            as_str(): "complex_vec_test",
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
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 440,
                                            end: 442,
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 449,
                                                            end: 452,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 453,
                                                                    end: 456,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 457,
                                                                end: 494,
                                                                as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 495,
                                                            end: 496,
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
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 497,
                                                                            end: 500,
                                                                            as_str(): "Vec",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 500,
                                                                                end: 502,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 502,
                                                                                    end: 505,
                                                                                    as_str(): "new",
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
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 505,
                                                                end: 507,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 507,
                                                            end: 508,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 513,
                                                            end: 516,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 517,
                                                                    end: 520,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 521,
                                                                end: 532,
                                                                as_str(): "inner_vec_1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 533,
                                                            end: 534,
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
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 535,
                                                                            end: 538,
                                                                            as_str(): "Vec",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 538,
                                                                                end: 540,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 540,
                                                                                    end: 543,
                                                                                    as_str(): "new",
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
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 543,
                                                                end: 545,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 545,
                                                            end: 546,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 551,
                                                            end: 554,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 555,
                                                                end: 572,
                                                                as_str(): "inner_inner_vec_1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 573,
                                                            end: 574,
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
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 575,
                                                                            end: 583,
                                                                            as_str(): "vec_from",
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
                                                                    Array(
                                                                        SquareBrackets {
                                                                            inner: Sequence(
                                                                                Punctuated {
                                                                                    value_separator_pairs: [
                                                                                        (
                                                                                            Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 585,
                                                                                                            end: 586,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                        parsed: 0,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                            CommaToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 586,
                                                                                                    end: 587,
                                                                                                    as_str(): ",",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        (
                                                                                            Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 588,
                                                                                                            end: 589,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                        parsed: 1,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                            CommaToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 589,
                                                                                                    end: 590,
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
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 591,
                                                                                                        end: 592,
                                                                                                        as_str(): "2",
                                                                                                    },
                                                                                                    parsed: 2,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 584,
                                                                                end: 593,
                                                                                as_str(): "[0, 1, 2]",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 583,
                                                                end: 594,
                                                                as_str(): "([0, 1, 2])",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 594,
                                                            end: 595,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 601,
                                                                        end: 612,
                                                                        as_str(): "inner_vec_1",
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 612,
                                                            end: 613,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 613,
                                                                end: 617,
                                                                as_str(): "push",
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
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 618,
                                                                                    end: 635,
                                                                                    as_str(): "inner_inner_vec_1",
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 617,
                                                            end: 636,
                                                            as_str(): "(inner_inner_vec_1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 636,
                                                            end: 637,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 642,
                                                                        end: 679,
                                                                        as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 679,
                                                            end: 680,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 680,
                                                                end: 684,
                                                                as_str(): "push",
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
                                                                Struct {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 685,
                                                                                    end: 695,
                                                                                    as_str(): "SomeStruct",
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
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 698,
                                                                                            end: 699,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    expr_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 699,
                                                                                                    end: 700,
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
                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                ),
                                                                                                                start: 701,
                                                                                                                end: 712,
                                                                                                                as_str(): "inner_vec_1",
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
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 696,
                                                                            end: 714,
                                                                            as_str(): "{ a: inner_vec_1 }",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 684,
                                                            end: 715,
                                                            as_str(): "(SomeStruct { a: inner_vec_1 })",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 715,
                                                            end: 716,
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
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 722,
                                                                        end: 728,
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
                                                                                target: MethodCall {
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 729,
                                                                                                        end: 740,
                                                                                                        as_str(): "inner_vec_1",
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
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 740,
                                                                                            end: 741,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    path_seg: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 741,
                                                                                                end: 744,
                                                                                                as_str(): "get",
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
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                ),
                                                                                                                start: 745,
                                                                                                                end: 746,
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
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 744,
                                                                                            end: 747,
                                                                                            as_str(): "(0)",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                dot_token: DotToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 747,
                                                                                        end: 748,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                path_seg: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 748,
                                                                                            end: 754,
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
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 754,
                                                                                        end: 756,
                                                                                        as_str(): "()",
                                                                                    },
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 756,
                                                                                    end: 757,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            path_seg: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 757,
                                                                                        end: 760,
                                                                                        as_str(): "get",
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
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 761,
                                                                                                        end: 762,
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
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 760,
                                                                                    end: 763,
                                                                                    as_str(): "(1)",
                                                                                },
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 763,
                                                                                end: 764,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        path_seg: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 764,
                                                                                    end: 770,
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 770,
                                                                                end: 772,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 773,
                                                                            end: 775,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 776,
                                                                                    end: 777,
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 728,
                                                            end: 778,
                                                            as_str(): "(inner_vec_1.get(0).unwrap().get(1).unwrap() == 1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 778,
                                                            end: 779,
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
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 784,
                                                                        end: 790,
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
                                                                                target: MethodCall {
                                                                                    target: FieldProjection {
                                                                                        target: MethodCall {
                                                                                            target: MethodCall {
                                                                                                target: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 791,
                                                                                                                    end: 828,
                                                                                                                    as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
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
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 828,
                                                                                                        end: 829,
                                                                                                        as_str(): ".",
                                                                                                    },
                                                                                                },
                                                                                                path_seg: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 829,
                                                                                                            end: 832,
                                                                                                            as_str(): "get",
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
                                                                                                            Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 833,
                                                                                                                            end: 834,
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
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 832,
                                                                                                        end: 835,
                                                                                                        as_str(): "(0)",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                            dot_token: DotToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 835,
                                                                                                    end: 836,
                                                                                                    as_str(): ".",
                                                                                                },
                                                                                            },
                                                                                            path_seg: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 836,
                                                                                                        end: 842,
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
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 842,
                                                                                                    end: 844,
                                                                                                    as_str(): "()",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                        dot_token: DotToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 844,
                                                                                                end: 845,
                                                                                                as_str(): ".",
                                                                                            },
                                                                                        },
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 845,
                                                                                                end: 846,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 846,
                                                                                            end: 847,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    path_seg: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 847,
                                                                                                end: 850,
                                                                                                as_str(): "get",
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
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                ),
                                                                                                                start: 851,
                                                                                                                end: 852,
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
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 850,
                                                                                            end: 853,
                                                                                            as_str(): "(0)",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                dot_token: DotToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
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
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
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
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 860,
                                                                                        end: 862,
                                                                                        as_str(): "()",
                                                                                    },
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 862,
                                                                                    end: 863,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            path_seg: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 863,
                                                                                        end: 866,
                                                                                        as_str(): "get",
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
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 867,
                                                                                                        end: 868,
                                                                                                        as_str(): "2",
                                                                                                    },
                                                                                                    parsed: 2,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 866,
                                                                                    end: 869,
                                                                                    as_str(): "(2)",
                                                                                },
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 869,
                                                                                end: 870,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        path_seg: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 870,
                                                                                    end: 876,
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 876,
                                                                                end: 878,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 879,
                                                                            end: 881,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 882,
                                                                                    end: 883,
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 790,
                                                            end: 884,
                                                            as_str(): "(exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 884,
                                                            end: 885,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 443,
                                        end: 887,
                                        as_str(): "{\n    let mut exp_vec_in_a_vec_in_a_struct_in_a_vec = Vec::new();\n    let mut inner_vec_1 = Vec::new();\n    let inner_inner_vec_1 = vec_from([0, 1, 2]);\n\n    inner_vec_1.push(inner_inner_vec_1);\n    exp_vec_in_a_vec_in_a_struct_in_a_vec.push(SomeStruct { a: inner_vec_1 });\n\n    assert(inner_vec_1.get(0).unwrap().get(1).unwrap() == 1);\n    assert(exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2);\n}",
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
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 889,
                                            end: 891,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 892,
                                            end: 919,
                                            as_str(): "simple_option_generics_test",
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
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 919,
                                            end: 921,
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
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 928,
                                                                        end: 934,
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
                                                                MethodCall {
                                                                    target: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 935,
                                                                                            end: 948,
                                                                                            as_str(): "get_an_option",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: Some(
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 948,
                                                                                                    end: 950,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            GenericArgs {
                                                                                                parameters: AngleBrackets {
                                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 950,
                                                                                                            end: 951,
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
                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 951,
                                                                                                                                end: 954,
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
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 954,
                                                                                                            end: 955,
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 955,
                                                                                end: 957,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    dot_token: DotToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 957,
                                                                            end: 958,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    path_seg: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 958,
                                                                                end: 965,
                                                                                as_str(): "is_none",
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
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 965,
                                                                            end: 967,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 934,
                                                            end: 968,
                                                            as_str(): "(get_an_option::<u64>().is_none())",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 968,
                                                            end: 969,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 922,
                                        end: 971,
                                        as_str(): "{\n    assert(get_an_option::<u64>().is_none());\n}",
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
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 973,
                                            end: 975,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 976,
                                            end: 980,
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
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 980,
                                            end: 982,
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
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 989,
                                                                        end: 1001,
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
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 1001,
                                                            end: 1003,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 1003,
                                                            end: 1004,
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
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1009,
                                                                        end: 1024,
                                                                        as_str(): "simple_vec_test",
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 1024,
                                                            end: 1026,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 1026,
                                                            end: 1027,
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
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1032,
                                                                        end: 1048,
                                                                        as_str(): "complex_vec_test",
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 1048,
                                                            end: 1050,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
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
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1056,
                                                                        end: 1083,
                                                                        as_str(): "simple_option_generics_test",
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 1083,
                                                            end: 1085,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 1085,
                                                            end: 1086,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 983,
                                        end: 1088,
                                        as_str(): "{\n    sell_product();\n    simple_vec_test();\n    complex_vec_test();\n    simple_option_generics_test();\n}",
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
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 1090,
                                            end: 1092,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 1093,
                                            end: 1105,
                                            as_str(): "sell_product",
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
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 1105,
                                            end: 1107,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 1108,
                                                    end: 1110,
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 1111,
                                                                end: 1119,
                                                                as_str(): "MyResult",
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1119,
                                                                                end: 1120,
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
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1120,
                                                                                                        end: 1124,
                                                                                                        as_str(): "bool",
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
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 1124,
                                                                                            end: 1125,
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
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1126,
                                                                                                    end: 1136,
                                                                                                    as_str(): "CustomType",
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1136,
                                                                                end: 1137,
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
                                            Expr {
                                                expr: If(
                                                    IfExpr {
                                                        if_token: IfToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 1144,
                                                                end: 1146,
                                                                as_str(): "if",
                                                            },
                                                        },
                                                        condition: Expr(
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1147,
                                                                            end: 1152,
                                                                            as_str(): "false",
                                                                        },
                                                                        kind: False,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        then_block: Braces {
                                                            inner: CodeBlockContents {
                                                                statements: [
                                                                    Expr {
                                                                        expr: Return {
                                                                            return_token: ReturnToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 1163,
                                                                                    end: 1169,
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
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1170,
                                                                                                        end: 1178,
                                                                                                        as_str(): "MyResult",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1178,
                                                                                                            end: 1180,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1180,
                                                                                                                end: 1183,
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
                                                                                                Struct {
                                                                                                    path: PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1184,
                                                                                                                    end: 1194,
                                                                                                                    as_str(): "CustomType",
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
                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1209,
                                                                                                                            end: 1213,
                                                                                                                            as_str(): "name",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    expr_opt: Some(
                                                                                                                        (
                                                                                                                            ColonToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1213,
                                                                                                                                    end: 1214,
                                                                                                                                    as_str(): ":",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            Literal(
                                                                                                                                String(
                                                                                                                                    LitString {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1215,
                                                                                                                                            end: 1220,
                                                                                                                                            as_str(): "\"foo\"",
                                                                                                                                        },
                                                                                                                                        parsed: "foo",
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1195,
                                                                                                            end: 1230,
                                                                                                            as_str(): "{\n            name: \"foo\"\n        }",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 1183,
                                                                                            end: 1231,
                                                                                            as_str(): "(CustomType {\n            name: \"foo\"\n        })",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        semicolon_token_opt: Some(
                                                                            SemicolonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 1231,
                                                                                    end: 1232,
                                                                                    as_str(): ";",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                                final_expr_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 1153,
                                                                end: 1238,
                                                                as_str(): "{\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    }",
                                                            },
                                                        },
                                                        else_opt: None,
                                                    },
                                                ),
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 1238,
                                                            end: 1239,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 1245,
                                                            end: 1251,
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1252,
                                                                                end: 1260,
                                                                                as_str(): "MyResult",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 1260,
                                                                                    end: 1262,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 1262,
                                                                                        end: 1264,
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
                                                                            Bool(
                                                                                LitBool {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 1265,
                                                                                        end: 1270,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                    kind: False,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1264,
                                                                    end: 1271,
                                                                    as_str(): "(false)",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 1271,
                                                            end: 1272,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 1138,
                                        end: 1274,
                                        as_str(): "{\n    if false {\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    };\n\n    return MyResult::Ok(false);\n}",
                                    },
                                },
                            },
                        ),
                    },
                ],
            },
            submodules: [
                (
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0eaa4c3f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                            ),
                            start: 8,
                            end: 13,
                            as_str(): "utils",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0eaa4c3f0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                ),
                                start: 8,
                                end: 13,
                                as_str(): "utils",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0eaa4c3f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0eaa4c3f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                            ),
                                            start: 8,
                                            end: 13,
                                            as_str(): "utils",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0eaa4c3f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                        ),
                                        start: 13,
                                        end: 14,
                                        as_str(): ";",
                                    },
                                },
                                items: [
                                    Annotated {
                                        attribute_list: [],
                                        value: Fn(
                                            ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: Some(
                                                        PubToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                ),
                                                                start: 16,
                                                                end: 19,
                                                                as_str(): "pub",
                                                            },
                                                        },
                                                    ),
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                            ),
                                                            start: 20,
                                                            end: 22,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                            ),
                                                            start: 23,
                                                            end: 31,
                                                            as_str(): "vec_from",
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
                                                                                    src (ptr): 0x00007fe0eaa4c3f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                    ),
                                                                                    start: 32,
                                                                                    end: 36,
                                                                                    as_str(): "vals",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        colon_token: ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                ),
                                                                                start: 36,
                                                                                end: 37,
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
                                                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                        ),
                                                                                                        start: 39,
                                                                                                        end: 42,
                                                                                                        as_str(): "u32",
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
                                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                            ),
                                                                                            start: 42,
                                                                                            end: 43,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                    length: Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                    ),
                                                                                                    start: 44,
                                                                                                    end: 45,
                                                                                                    as_str(): "3",
                                                                                                },
                                                                                                parsed: 3,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eaa4c3f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                    ),
                                                                                    start: 38,
                                                                                    end: 46,
                                                                                    as_str(): "[u32; 3]",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                            ),
                                                            start: 31,
                                                            end: 47,
                                                            as_str(): "(vals: [u32; 3])",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eaa4c3f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 50,
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
                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                ),
                                                                                start: 51,
                                                                                end: 54,
                                                                                as_str(): "Vec",
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
                                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                ),
                                                                                                start: 54,
                                                                                                end: 55,
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
                                                                                                                    src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                                    ),
                                                                                                                    start: 55,
                                                                                                                    end: 58,
                                                                                                                    as_str(): "u32",
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
                                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                ),
                                                                                                start: 58,
                                                                                                end: 59,
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
                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                            ),
                                                                            start: 66,
                                                                            end: 69,
                                                                            as_str(): "let",
                                                                        },
                                                                    },
                                                                    pattern: Var {
                                                                        reference: None,
                                                                        mutable: Some(
                                                                            MutToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eaa4c3f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                    ),
                                                                                    start: 70,
                                                                                    end: 73,
                                                                                    as_str(): "mut",
                                                                                },
                                                                            },
                                                                        ),
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                ),
                                                                                start: 74,
                                                                                end: 77,
                                                                                as_str(): "vec",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    ty_opt: None,
                                                                    eq_token: EqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                            ),
                                                                            start: 78,
                                                                            end: 79,
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
                                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                            ),
                                                                                            start: 80,
                                                                                            end: 83,
                                                                                            as_str(): "Vec",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                ),
                                                                                                start: 83,
                                                                                                end: 85,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                    ),
                                                                                                    start: 85,
                                                                                                    end: 88,
                                                                                                    as_str(): "new",
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
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                ),
                                                                                start: 88,
                                                                                end: 90,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    semicolon_token: SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                            ),
                                                                            start: 90,
                                                                            end: 91,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                            Expr {
                                                                expr: MethodCall {
                                                                    target: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                        ),
                                                                                        start: 96,
                                                                                        end: 99,
                                                                                        as_str(): "vec",
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
                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                            ),
                                                                            start: 99,
                                                                            end: 100,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    path_seg: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                ),
                                                                                start: 100,
                                                                                end: 104,
                                                                                as_str(): "push",
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
                                                                                Index {
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                        ),
                                                                                                        start: 105,
                                                                                                        end: 109,
                                                                                                        as_str(): "vals",
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
                                                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                        ),
                                                                                                        start: 110,
                                                                                                        end: 111,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                    parsed: 0,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                            ),
                                                                                            start: 109,
                                                                                            end: 112,
                                                                                            as_str(): "[0]",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                            ),
                                                                            start: 104,
                                                                            end: 113,
                                                                            as_str(): "(vals[0])",
                                                                        },
                                                                    },
                                                                },
                                                                semicolon_token_opt: Some(
                                                                    SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                            ),
                                                                            start: 113,
                                                                            end: 114,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            Expr {
                                                                expr: MethodCall {
                                                                    target: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                        ),
                                                                                        start: 119,
                                                                                        end: 122,
                                                                                        as_str(): "vec",
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
                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                            ),
                                                                            start: 122,
                                                                            end: 123,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    path_seg: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                ),
                                                                                start: 123,
                                                                                end: 127,
                                                                                as_str(): "push",
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
                                                                                Index {
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                        ),
                                                                                                        start: 128,
                                                                                                        end: 132,
                                                                                                        as_str(): "vals",
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
                                                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                        ),
                                                                                                        start: 133,
                                                                                                        end: 134,
                                                                                                        as_str(): "1",
                                                                                                    },
                                                                                                    parsed: 1,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                            ),
                                                                                            start: 132,
                                                                                            end: 135,
                                                                                            as_str(): "[1]",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                            ),
                                                                            start: 127,
                                                                            end: 136,
                                                                            as_str(): "(vals[1])",
                                                                        },
                                                                    },
                                                                },
                                                                semicolon_token_opt: Some(
                                                                    SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                            ),
                                                                            start: 136,
                                                                            end: 137,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            Expr {
                                                                expr: MethodCall {
                                                                    target: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                        ),
                                                                                        start: 142,
                                                                                        end: 145,
                                                                                        as_str(): "vec",
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
                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                            ),
                                                                            start: 145,
                                                                            end: 146,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    path_seg: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                ),
                                                                                start: 146,
                                                                                end: 150,
                                                                                as_str(): "push",
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
                                                                                Index {
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                        ),
                                                                                                        start: 151,
                                                                                                        end: 155,
                                                                                                        as_str(): "vals",
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
                                                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                        ),
                                                                                                        start: 156,
                                                                                                        end: 157,
                                                                                                        as_str(): "2",
                                                                                                    },
                                                                                                    parsed: 2,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                            ),
                                                                                            start: 155,
                                                                                            end: 158,
                                                                                            as_str(): "[2]",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                            ),
                                                                            start: 150,
                                                                            end: 159,
                                                                            as_str(): "(vals[2])",
                                                                        },
                                                                    },
                                                                },
                                                                semicolon_token_opt: Some(
                                                                    SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                            ),
                                                                            start: 159,
                                                                            end: 160,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        ],
                                                        final_expr_opt: Some(
                                                            Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                ),
                                                                                start: 165,
                                                                                end: 168,
                                                                                as_str(): "vec",
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
                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                        ),
                                                        start: 60,
                                                        end: 170,
                                                        as_str(): "{\n    let mut vec = Vec::new();\n    vec.push(vals[0]);\n    vec.push(vals[1]);\n    vec.push(vals[2]);\n    vec\n}",
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
                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                ),
                                                                start: 172,
                                                                end: 175,
                                                                as_str(): "pub",
                                                            },
                                                        },
                                                    ),
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                            ),
                                                            start: 176,
                                                            end: 178,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                            ),
                                                            start: 179,
                                                            end: 192,
                                                            as_str(): "get_an_option",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: Some(
                                                        GenericParams {
                                                            parameters: AngleBrackets {
                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                        ),
                                                                        start: 192,
                                                                        end: 193,
                                                                        as_str(): "<",
                                                                    },
                                                                },
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                ),
                                                                                start: 193,
                                                                                end: 194,
                                                                                as_str(): "T",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                },
                                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                        ),
                                                                        start: 194,
                                                                        end: 195,
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
                                                                final_value_opt: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eaa4c3f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                            ),
                                                            start: 195,
                                                            end: 197,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eaa4c3f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                    ),
                                                                    start: 198,
                                                                    end: 200,
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
                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                ),
                                                                                start: 201,
                                                                                end: 207,
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
                                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                ),
                                                                                                start: 207,
                                                                                                end: 208,
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
                                                                                                                    src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                                    ),
                                                                                                                    start: 208,
                                                                                                                    end: 209,
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
                                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                                ),
                                                                                                start: 209,
                                                                                                end: 210,
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
                                                            Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eaa4c3f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                ),
                                                                                start: 217,
                                                                                end: 223,
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
                                                                                    src (ptr): 0x00007fe0eaa4c3f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                    ),
                                                                                    start: 223,
                                                                                    end: 225,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                                        ),
                                                                                        start: 225,
                                                                                        end: 229,
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
                                                        src (ptr): 0x00007fe0eaa4c3f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                        ),
                                                        start: 211,
                                                        end: 231,
                                                        as_str(): "{\n    Option::None\n}",
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
                ),
            ],
        },
    },
)
