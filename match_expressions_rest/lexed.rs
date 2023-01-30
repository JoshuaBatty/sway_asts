Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe092bf2270,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe092bf2270,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
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
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 21,
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
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 26,
                                                                end: 27,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 27,
                                                                end: 28,
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
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 29,
                                                                            end: 32,
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
                                                        src (ptr): 0x00007fe092bf2270,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                        ),
                                                        start: 32,
                                                        end: 33,
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
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 36,
                                                            end: 37,
                                                            as_str(): "y",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 37,
                                                            end: 38,
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
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 39,
                                                                        end: 42,
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
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 22,
                                        end: 44,
                                        as_str(): "{\n  x: u64,\n  y: u64\n}",
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
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 46,
                                        end: 52,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 53,
                                        end: 60,
                                        as_str(): "Point3D",
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
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 65,
                                                                end: 66,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 66,
                                                                end: 67,
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
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 68,
                                                                            end: 71,
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
                                                        src (ptr): 0x00007fe092bf2270,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                        ),
                                                        start: 71,
                                                        end: 72,
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
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 75,
                                                                end: 76,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 76,
                                                                end: 77,
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
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 78,
                                                                            end: 81,
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
                                                        src (ptr): 0x00007fe092bf2270,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                        ),
                                                        start: 81,
                                                        end: 82,
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
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 85,
                                                            end: 86,
                                                            as_str(): "z",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 86,
                                                            end: 87,
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
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 88,
                                                                        end: 91,
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
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 61,
                                        end: 93,
                                        as_str(): "{\n  x: u64,\n  y: u64,\n  z: u64\n}",
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
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 101,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 102,
                                        end: 106,
                                        as_str(): "Line",
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
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 113,
                                                                end: 115,
                                                                as_str(): "p1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 115,
                                                                end: 116,
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
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 117,
                                                                            end: 122,
                                                                            as_str(): "Point",
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
                                                        src (ptr): 0x00007fe092bf2270,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                        ),
                                                        start: 122,
                                                        end: 123,
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
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 128,
                                                            end: 130,
                                                            as_str(): "p2",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 130,
                                                            end: 131,
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
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 132,
                                                                        end: 137,
                                                                        as_str(): "Point",
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
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 107,
                                        end: 139,
                                        as_str(): "{\n    p1: Point,\n    p2: Point\n}",
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
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 141,
                                        end: 145,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 146,
                                        end: 150,
                                        as_str(): "Kind",
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
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 157,
                                                                end: 162,
                                                                as_str(): "Point",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 162,
                                                                end: 163,
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
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 164,
                                                                            end: 169,
                                                                            as_str(): "Point",
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
                                                        src (ptr): 0x00007fe092bf2270,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                        ),
                                                        start: 169,
                                                        end: 170,
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
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 175,
                                                                end: 182,
                                                                as_str(): "Point3D",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 182,
                                                                end: 183,
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
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 184,
                                                                            end: 191,
                                                                            as_str(): "Point3D",
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
                                                        src (ptr): 0x00007fe092bf2270,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                        ),
                                                        start: 191,
                                                        end: 192,
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
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 197,
                                                            end: 201,
                                                            as_str(): "Line",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 201,
                                                            end: 202,
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
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 203,
                                                                        end: 207,
                                                                        as_str(): "Line",
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
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 151,
                                        end: 209,
                                        as_str(): "{\n    Point: Point,\n    Point3D: Point3D,\n    Line: Line\n}",
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
                                            src (ptr): 0x00007fe092bf2270,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                            ),
                                            start: 211,
                                            end: 213,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe092bf2270,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                            ),
                                            start: 214,
                                            end: 218,
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
                                            src (ptr): 0x00007fe092bf2270,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                            ),
                                            start: 218,
                                            end: 220,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe092bf2270,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                    ),
                                                    start: 221,
                                                    end: 223,
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
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 224,
                                                                end: 227,
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
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 234,
                                                            end: 237,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 238,
                                                                end: 239,
                                                                as_str(): "p",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 240,
                                                            end: 241,
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
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 242,
                                                                        end: 247,
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
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 258,
                                                                                    end: 259,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 259,
                                                                                            end: 260,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                    ),
                                                                                                    start: 261,
                                                                                                    end: 262,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U64,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                            ),
                                                                                                            start: 262,
                                                                                                            end: 265,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 265,
                                                                                end: 266,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 275,
                                                                                    end: 276,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 276,
                                                                                            end: 277,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                    ),
                                                                                                    start: 278,
                                                                                                    end: 279,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                                parsed: 2,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U64,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                            ),
                                                                                                            start: 279,
                                                                                                            end: 282,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 282,
                                                                                end: 283,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 248,
                                                                end: 289,
                                                                as_str(): "{\n        x: 1u64,\n        y: 2u64,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 289,
                                                            end: 290,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Match {
                                                    match_token: MatchToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 296,
                                                            end: 301,
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
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 302,
                                                                        end: 303,
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
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 314,
                                                                                    end: 319,
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
                                                                                    Field {
                                                                                        field_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                ),
                                                                                                start: 322,
                                                                                                end: 323,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        pattern_opt: None,
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 323,
                                                                                            end: 324,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: Some(
                                                                                Rest {
                                                                                    token: DoubleDotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 325,
                                                                                            end: 327,
                                                                                            as_str(): "..",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 320,
                                                                            end: 329,
                                                                            as_str(): "{ x, .. }",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 330,
                                                                        end: 332,
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
                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                    ),
                                                                                                    start: 335,
                                                                                                    end: 336,
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
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 333,
                                                                            end: 338,
                                                                            as_str(): "{ x }",
                                                                        },
                                                                    },
                                                                    comma_token_opt: Some(
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 338,
                                                                                end: 339,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 304,
                                                            end: 345,
                                                            as_str(): "{\n        Point { x, .. } => { x },\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 345,
                                                            end: 346,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 352,
                                                            end: 355,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 356,
                                                                end: 358,
                                                                as_str(): "p2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 359,
                                                            end: 360,
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
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 361,
                                                                        end: 368,
                                                                        as_str(): "Point3D",
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
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 379,
                                                                                    end: 380,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 380,
                                                                                            end: 381,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                    ),
                                                                                                    start: 382,
                                                                                                    end: 383,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U64,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                            ),
                                                                                                            start: 383,
                                                                                                            end: 386,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 386,
                                                                                end: 387,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 396,
                                                                                    end: 397,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                    ),
                                                                                                    start: 399,
                                                                                                    end: 400,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                                parsed: 2,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U64,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                            ),
                                                                                                            start: 400,
                                                                                                            end: 403,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 403,
                                                                                end: 404,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 413,
                                                                                    end: 414,
                                                                                    as_str(): "z",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 414,
                                                                                            end: 415,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                    ),
                                                                                                    start: 416,
                                                                                                    end: 417,
                                                                                                    as_str(): "3",
                                                                                                },
                                                                                                parsed: 3,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U64,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                            ),
                                                                                                            start: 417,
                                                                                                            end: 420,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 420,
                                                                                end: 421,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 369,
                                                                end: 427,
                                                                as_str(): "{\n        x: 1u64,\n        y: 2u64,\n        z: 3u64,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 427,
                                                            end: 428,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Match {
                                                    match_token: MatchToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 434,
                                                            end: 439,
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
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 440,
                                                                        end: 442,
                                                                        as_str(): "p2",
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
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 453,
                                                                                    end: 460,
                                                                                    as_str(): "Point3D",
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
                                                                                    Field {
                                                                                        field_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                ),
                                                                                                start: 463,
                                                                                                end: 464,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        pattern_opt: None,
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 464,
                                                                                            end: 465,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: Some(
                                                                                Rest {
                                                                                    token: DoubleDotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 466,
                                                                                            end: 468,
                                                                                            as_str(): "..",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 461,
                                                                            end: 470,
                                                                            as_str(): "{ x, .. }",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 471,
                                                                        end: 473,
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
                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                    ),
                                                                                                    start: 476,
                                                                                                    end: 477,
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
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 474,
                                                                            end: 479,
                                                                            as_str(): "{ x }",
                                                                        },
                                                                    },
                                                                    comma_token_opt: Some(
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 479,
                                                                                end: 480,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 443,
                                                            end: 486,
                                                            as_str(): "{\n        Point3D { x, .. } => { x },\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 486,
                                                            end: 487,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 493,
                                                            end: 496,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 497,
                                                                end: 498,
                                                                as_str(): "l",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 499,
                                                            end: 500,
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
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 501,
                                                                        end: 505,
                                                                        as_str(): "Line",
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
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 508,
                                                                                    end: 510,
                                                                                    as_str(): "p1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 510,
                                                                                            end: 511,
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
                                                                                                        src (ptr): 0x00007fe092bf2270,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                        ),
                                                                                                        start: 512,
                                                                                                        end: 513,
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
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 513,
                                                                                end: 514,
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
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 515,
                                                                                end: 517,
                                                                                as_str(): "p2",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092bf2270,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                        ),
                                                                                        start: 517,
                                                                                        end: 518,
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
                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                    ),
                                                                                                    start: 519,
                                                                                                    end: 520,
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
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 506,
                                                                end: 522,
                                                                as_str(): "{ p1: p, p2: p }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 522,
                                                            end: 523,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Match {
                                                    match_token: MatchToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 529,
                                                            end: 534,
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
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 535,
                                                                        end: 536,
                                                                        as_str(): "l",
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
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 547,
                                                                                    end: 551,
                                                                                    as_str(): "Line",
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
                                                                                    Field {
                                                                                        field_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                ),
                                                                                                start: 554,
                                                                                                end: 556,
                                                                                                as_str(): "p1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        pattern_opt: None,
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 556,
                                                                                            end: 557,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: Some(
                                                                                Rest {
                                                                                    token: DoubleDotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 558,
                                                                                            end: 560,
                                                                                            as_str(): "..",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 552,
                                                                            end: 562,
                                                                            as_str(): "{ p1, .. }",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 563,
                                                                        end: 565,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Block {
                                                                    block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 566,
                                                                            end: 568,
                                                                            as_str(): "{}",
                                                                        },
                                                                    },
                                                                    comma_token_opt: None,
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 537,
                                                            end: 574,
                                                            as_str(): "{\n        Line { p1, .. } => {}\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
                                            },
                                            Expr {
                                                expr: Match {
                                                    match_token: MatchToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 580,
                                                            end: 585,
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
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 586,
                                                                        end: 587,
                                                                        as_str(): "l",
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
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 598,
                                                                                    end: 602,
                                                                                    as_str(): "Line",
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
                                                                                    Field {
                                                                                        field_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                ),
                                                                                                start: 605,
                                                                                                end: 607,
                                                                                                as_str(): "p1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        pattern_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092bf2270,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                        ),
                                                                                                        start: 607,
                                                                                                        end: 608,
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
                                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 609,
                                                                                                                    end: 614,
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
                                                                                                            value_separator_pairs: [],
                                                                                                            final_value_opt: Some(
                                                                                                                Rest {
                                                                                                                    token: DoubleDotToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 617,
                                                                                                                            end: 619,
                                                                                                                            as_str(): "..",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                            ),
                                                                                                            start: 615,
                                                                                                            end: 621,
                                                                                                            as_str(): "{ .. }",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 621,
                                                                                            end: 622,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: Some(
                                                                                Field {
                                                                                    field_name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 623,
                                                                                            end: 625,
                                                                                            as_str(): "p2",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    pattern_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                    ),
                                                                                                    start: 625,
                                                                                                    end: 626,
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
                                                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                ),
                                                                                                                start: 627,
                                                                                                                end: 632,
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
                                                                                                        value_separator_pairs: [],
                                                                                                        final_value_opt: Some(
                                                                                                            Rest {
                                                                                                                token: DoubleDotToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe092bf2270,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 635,
                                                                                                                        end: 637,
                                                                                                                        as_str(): "..",
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092bf2270,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                        ),
                                                                                                        start: 633,
                                                                                                        end: 639,
                                                                                                        as_str(): "{ .. }",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 603,
                                                                            end: 641,
                                                                            as_str(): "{ p1: Point { .. }, p2: Point { .. } }",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 642,
                                                                        end: 644,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Block {
                                                                    block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 645,
                                                                            end: 647,
                                                                            as_str(): "{}",
                                                                        },
                                                                    },
                                                                    comma_token_opt: None,
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 588,
                                                            end: 653,
                                                            as_str(): "{\n        Line { p1: Point { .. }, p2: Point { .. } } => {}\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 659,
                                                            end: 662,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 663,
                                                                end: 664,
                                                                as_str(): "k",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 665,
                                                            end: 666,
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
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 667,
                                                                            end: 671,
                                                                            as_str(): "Kind",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 671,
                                                                                end: 673,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 673,
                                                                                    end: 678,
                                                                                    as_str(): "Point",
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
                                                                                        src (ptr): 0x00007fe092bf2270,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                        ),
                                                                                        start: 679,
                                                                                        end: 680,
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
                                                                src (ptr): 0x00007fe092bf2270,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                ),
                                                                start: 678,
                                                                end: 681,
                                                                as_str(): "(p)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 681,
                                                            end: 682,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Match {
                                                    match_token: MatchToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 688,
                                                            end: 693,
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
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 694,
                                                                        end: 695,
                                                                        as_str(): "k",
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
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 706,
                                                                                    end: 710,
                                                                                    as_str(): "Kind",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092bf2270,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                        ),
                                                                                        start: 710,
                                                                                        end: 712,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 712,
                                                                                            end: 717,
                                                                                            as_str(): "Point",
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
                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                    ),
                                                                                                    start: 718,
                                                                                                    end: 723,
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
                                                                                                    Field {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                ),
                                                                                                                start: 726,
                                                                                                                end: 727,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        pattern_opt: None,
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                            ),
                                                                                                            start: 727,
                                                                                                            end: 728,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            final_value_opt: Some(
                                                                                                Rest {
                                                                                                    token: DoubleDotToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                            ),
                                                                                                            start: 729,
                                                                                                            end: 731,
                                                                                                            as_str(): "..",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 724,
                                                                                            end: 733,
                                                                                            as_str(): "{ x, .. }",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 717,
                                                                            end: 734,
                                                                            as_str(): "(Point { x, .. })",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 735,
                                                                        end: 737,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Block {
                                                                    block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 738,
                                                                            end: 740,
                                                                            as_str(): "{}",
                                                                        },
                                                                    },
                                                                    comma_token_opt: Some(
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 740,
                                                                                end: 741,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
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
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 750,
                                                                                    end: 754,
                                                                                    as_str(): "Kind",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092bf2270,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                        ),
                                                                                        start: 754,
                                                                                        end: 756,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 756,
                                                                                            end: 763,
                                                                                            as_str(): "Point3D",
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
                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                    ),
                                                                                                    start: 764,
                                                                                                    end: 771,
                                                                                                    as_str(): "Point3D",
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
                                                                                                    Field {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                ),
                                                                                                                start: 774,
                                                                                                                end: 775,
                                                                                                                as_str(): "z",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        pattern_opt: None,
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                            ),
                                                                                                            start: 775,
                                                                                                            end: 776,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            final_value_opt: Some(
                                                                                                Rest {
                                                                                                    token: DoubleDotToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                            ),
                                                                                                            start: 777,
                                                                                                            end: 779,
                                                                                                            as_str(): "..",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 772,
                                                                                            end: 781,
                                                                                            as_str(): "{ z, .. }",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 763,
                                                                            end: 782,
                                                                            as_str(): "(Point3D { z, .. })",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 783,
                                                                        end: 785,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Block {
                                                                    block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 786,
                                                                            end: 788,
                                                                            as_str(): "{}",
                                                                        },
                                                                    },
                                                                    comma_token_opt: Some(
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 788,
                                                                                end: 789,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
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
                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                    ),
                                                                                    start: 798,
                                                                                    end: 802,
                                                                                    as_str(): "Kind",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092bf2270,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                        ),
                                                                                        start: 802,
                                                                                        end: 804,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 804,
                                                                                            end: 808,
                                                                                            as_str(): "Line",
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
                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                    ),
                                                                                                    start: 809,
                                                                                                    end: 813,
                                                                                                    as_str(): "Line",
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
                                                                                                    Field {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                ),
                                                                                                                start: 816,
                                                                                                                end: 818,
                                                                                                                as_str(): "p1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        pattern_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe092bf2270,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 818,
                                                                                                                        end: 819,
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
                                                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 820,
                                                                                                                                    end: 825,
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
                                                                                                                            value_separator_pairs: [],
                                                                                                                            final_value_opt: Some(
                                                                                                                                Rest {
                                                                                                                                    token: DoubleDotToken {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 828,
                                                                                                                                            end: 830,
                                                                                                                                            as_str(): "..",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        },
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 826,
                                                                                                                            end: 832,
                                                                                                                            as_str(): "{ .. }",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                            ),
                                                                                                            start: 832,
                                                                                                            end: 833,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            final_value_opt: Some(
                                                                                                Field {
                                                                                                    field_name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                            ),
                                                                                                            start: 834,
                                                                                                            end: 836,
                                                                                                            as_str(): "p2",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    pattern_opt: Some(
                                                                                                        (
                                                                                                            ColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe092bf2270,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 836,
                                                                                                                    end: 837,
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
                                                                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 838,
                                                                                                                                end: 843,
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
                                                                                                                        value_separator_pairs: [],
                                                                                                                        final_value_opt: Some(
                                                                                                                            Rest {
                                                                                                                                token: DoubleDotToken {
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe092bf2270,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 846,
                                                                                                                                        end: 848,
                                                                                                                                        as_str(): "..",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe092bf2270,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 844,
                                                                                                                        end: 850,
                                                                                                                        as_str(): "{ .. }",
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092bf2270,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                            ),
                                                                                            start: 814,
                                                                                            end: 852,
                                                                                            as_str(): "{ p1: Point { .. }, p2: Point { .. } }",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 808,
                                                                            end: 853,
                                                                            as_str(): "(Line { p1: Point { .. }, p2: Point { .. } })",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092bf2270,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                        ),
                                                                        start: 854,
                                                                        end: 856,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Block {
                                                                    block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092bf2270,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                            ),
                                                                            start: 857,
                                                                            end: 859,
                                                                            as_str(): "{}",
                                                                        },
                                                                    },
                                                                    comma_token_opt: Some(
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092bf2270,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                                                ),
                                                                                start: 859,
                                                                                end: 860,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 696,
                                                            end: 866,
                                                            as_str(): "{\n        Kind::Point(Point { x, .. }) => {},\n        Kind::Point3D(Point3D { z, .. }) => {},\n        Kind::Line(Line { p1: Point { .. }, p2: Point { .. } }) => {},\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092bf2270,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                                            ),
                                                            start: 872,
                                                            end: 873,
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
                                        src (ptr): 0x00007fe092bf2270,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcLAswV/match_expressions_rest/src/main.sw",
                                        ),
                                        start: 228,
                                        end: 875,
                                        as_str(): "{\n    let p = Point {\n        x: 1u64,\n        y: 2u64,\n    };\n\n    match p {\n        Point { x, .. } => { x },\n    };\n\n    let p2 = Point3D {\n        x: 1u64,\n        y: 2u64,\n        z: 3u64,\n    };\n\n    match p2 {\n        Point3D { x, .. } => { x },\n    };\n\n    let l = Line { p1: p, p2: p };\n\n    match l {\n        Line { p1, .. } => {}\n    }\n\n    match l {\n        Line { p1: Point { .. }, p2: Point { .. } } => {}\n    }\n\n    let k = Kind::Point(p);\n\n    match k {\n        Kind::Point(Point { x, .. }) => {},\n        Kind::Point3D(Point3D { z, .. }) => {},\n        Kind::Line(Line { p1: Point { .. }, p2: Point { .. } }) => {},\n    }\n\n    0\n}",
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
