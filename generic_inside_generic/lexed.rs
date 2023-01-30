Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0fd5d5ae0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0fd5d5ae0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
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
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 24,
                                        as_str(): "Generic1",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 25,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 25,
                                                            end: 26,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 26,
                                                    end: 27,
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
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 34,
                                                                end: 35,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 35,
                                                                end: 36,
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
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 37,
                                                                            end: 38,
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
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                        ),
                                                        start: 38,
                                                        end: 39,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 28,
                                        end: 41,
                                        as_str(): "{\n    a: T,\n}",
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
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 49,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 50,
                                        end: 58,
                                        as_str(): "Generic2",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 58,
                                                    end: 59,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 59,
                                                            end: 60,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 60,
                                                    end: 61,
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
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 68,
                                                                end: 69,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
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
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 71,
                                                                            end: 79,
                                                                            as_str(): "Generic1",
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
                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                            ),
                                                                                            start: 79,
                                                                                            end: 80,
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
                                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                ),
                                                                                                                start: 80,
                                                                                                                end: 81,
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
                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 82,
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
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
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
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 62,
                                        end: 85,
                                        as_str(): "{\n    b: Generic1<T>,\n}",
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
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 87,
                                        end: 91,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 92,
                                        end: 100,
                                        as_str(): "Generic3",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 100,
                                                    end: 101,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 101,
                                                            end: 102,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 102,
                                                    end: 103,
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
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 110,
                                                                end: 111,
                                                                as_str(): "A",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 111,
                                                                end: 112,
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
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 113,
                                                                            end: 114,
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
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                        ),
                                                        start: 114,
                                                        end: 115,
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
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 120,
                                                            end: 121,
                                                            as_str(): "B",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 121,
                                                            end: 122,
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
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 123,
                                                                        end: 124,
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
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 104,
                                        end: 126,
                                        as_str(): "{\n    A: T,\n    B: T\n}",
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
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 128,
                                        end: 132,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 133,
                                        end: 141,
                                        as_str(): "Generic4",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 141,
                                                    end: 142,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 142,
                                                            end: 143,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 143,
                                                    end: 144,
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
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 151,
                                                                end: 152,
                                                                as_str(): "C",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 152,
                                                                end: 153,
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
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 154,
                                                                            end: 162,
                                                                            as_str(): "Generic3",
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
                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                            ),
                                                                                            start: 162,
                                                                                            end: 163,
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
                                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                ),
                                                                                                                start: 163,
                                                                                                                end: 164,
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
                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                            ),
                                                                                            start: 164,
                                                                                            end: 165,
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
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                        ),
                                                        start: 165,
                                                        end: 166,
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
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 171,
                                                            end: 172,
                                                            as_str(): "D",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 173,
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
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 174,
                                                                        end: 182,
                                                                        as_str(): "Generic3",
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
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 182,
                                                                                        end: 183,
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
                                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 183,
                                                                                                            end: 184,
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
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 184,
                                                                                        end: 185,
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
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 145,
                                        end: 187,
                                        as_str(): "{\n    C: Generic3<T>,\n    D: Generic3<T>\n}",
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
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 189,
                                            end: 191,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 192,
                                            end: 196,
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
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 196,
                                            end: 198,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 199,
                                                    end: 201,
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
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 202,
                                                                end: 205,
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
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 212,
                                                            end: 215,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 216,
                                                                end: 217,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 218,
                                                            end: 219,
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
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 220,
                                                                        end: 228,
                                                                        as_str(): "Generic1",
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
                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                ),
                                                                                start: 239,
                                                                                end: 240,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 240,
                                                                                        end: 241,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                ),
                                                                                                start: 242,
                                                                                                end: 243,
                                                                                                as_str(): "7",
                                                                                            },
                                                                                            parsed: 7,
                                                                                            ty_opt: Some(
                                                                                                (
                                                                                                    U64,
                                                                                                    Span {
                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 243,
                                                                                                        end: 246,
                                                                                                        as_str(): "u64",
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 229,
                                                                end: 252,
                                                                as_str(): "{\n        a: 7u64\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 252,
                                                            end: 253,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 258,
                                                            end: 261,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 262,
                                                                end: 263,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 264,
                                                            end: 265,
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
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 266,
                                                                        end: 274,
                                                                        as_str(): "Generic2",
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
                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                ),
                                                                                start: 285,
                                                                                end: 286,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 286,
                                                                                        end: 287,
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
                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 288,
                                                                                                    end: 289,
                                                                                                    as_str(): "a",
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
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 275,
                                                                end: 295,
                                                                as_str(): "{\n        b: a\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 295,
                                                            end: 296,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 301,
                                                            end: 304,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 305,
                                                                end: 306,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 307,
                                                            end: 308,
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
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 309,
                                                                            end: 317,
                                                                            as_str(): "Generic3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                ),
                                                                                start: 317,
                                                                                end: 319,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 319,
                                                                                    end: 320,
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 321,
                                                                                        end: 322,
                                                                                        as_str(): "b",
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
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 320,
                                                                end: 323,
                                                                as_str(): "(b)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 323,
                                                            end: 324,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 329,
                                                            end: 332,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 333,
                                                                end: 334,
                                                                as_str(): "d",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 335,
                                                            end: 336,
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
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 337,
                                                                            end: 345,
                                                                            as_str(): "Generic4",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                ),
                                                                                start: 345,
                                                                                end: 347,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 347,
                                                                                    end: 348,
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 349,
                                                                                        end: 350,
                                                                                        as_str(): "c",
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
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 348,
                                                                end: 351,
                                                                as_str(): "(c)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 351,
                                                            end: 352,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Match {
                                                match_token: MatchToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                        ),
                                                        start: 358,
                                                        end: 363,
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
                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                    ),
                                                                    start: 364,
                                                                    end: 365,
                                                                    as_str(): "d",
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
                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                ),
                                                                                start: 376,
                                                                                end: 384,
                                                                                as_str(): "Generic4",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 384,
                                                                                    end: 386,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 386,
                                                                                        end: 387,
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
                                                                            Constructor {
                                                                                path: PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                ),
                                                                                                start: 401,
                                                                                                end: 409,
                                                                                                as_str(): "Generic3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 409,
                                                                                                    end: 411,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 411,
                                                                                                        end: 412,
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
                                                                                            Struct {
                                                                                                path: PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                ),
                                                                                                                start: 430,
                                                                                                                end: 438,
                                                                                                                as_str(): "Generic2",
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
                                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 461,
                                                                                                                        end: 462,
                                                                                                                        as_str(): "b",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                pattern_opt: Some(
                                                                                                                    (
                                                                                                                        ColonToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 462,
                                                                                                                                end: 463,
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
                                                                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 464,
                                                                                                                                            end: 472,
                                                                                                                                            as_str(): "Generic1",
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
                                                                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 499,
                                                                                                                                                    end: 500,
                                                                                                                                                    as_str(): "a",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            pattern_opt: None,
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                },
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 473,
                                                                                                                                    end: 522,
                                                                                                                                    as_str(): "{\n                        a\n                    }",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 439,
                                                                                                        end: 540,
                                                                                                        as_str(): "{\n                    b: Generic1 {\n                        a\n                    }\n                }",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 412,
                                                                                        end: 554,
                                                                                        as_str(): "(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 387,
                                                                        end: 564,
                                                                        as_str(): "(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        )",
                                                                    },
                                                                },
                                                            },
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                    ),
                                                                    start: 565,
                                                                    end: 567,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Block {
                                                                block: Braces {
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
                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                ),
                                                                                                start: 570,
                                                                                                end: 571,
                                                                                                as_str(): "a",
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
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 568,
                                                                        end: 573,
                                                                        as_str(): "{ a }",
                                                                    },
                                                                },
                                                                comma_token_opt: Some(
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 573,
                                                                            end: 574,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        MatchBranch {
                                                            pattern: Wildcard {
                                                                underscore_token: UnderscoreToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 583,
                                                                        end: 584,
                                                                        as_str(): "_",
                                                                    },
                                                                },
                                                            },
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                    ),
                                                                    start: 585,
                                                                    end: 587,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Block {
                                                                block: Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                            ),
                                                                                            start: 590,
                                                                                            end: 591,
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
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 588,
                                                                        end: 593,
                                                                        as_str(): "{ 0 }",
                                                                    },
                                                                },
                                                                comma_token_opt: None,
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                        ),
                                                        start: 366,
                                                        end: 599,
                                                        as_str(): "{\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 206,
                                        end: 601,
                                        as_str(): "{\n    let a = Generic1 {\n        a: 7u64\n    };\n    let b = Generic2 {\n        b: a\n    };\n    let c = Generic3::B(b);\n    let d = Generic4::C(c);\n\n    match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }\n}",
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
