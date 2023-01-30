Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0f8c2c380,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0f8c2c380,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
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
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 30,
                                        as_str(): "DoubleIdentity",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 30,
                                                    end: 31,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 31,
                                                                end: 32,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 32,
                                                                end: 33,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 34,
                                                            end: 35,
                                                            as_str(): "F",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 35,
                                                    end: 36,
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
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 41,
                                                                end: 46,
                                                                as_str(): "first",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 46,
                                                                end: 47,
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
                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                            ),
                                                                            start: 48,
                                                                            end: 49,
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
                                                        src (ptr): 0x00007fe0f8c2c380,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                        ),
                                                        start: 49,
                                                        end: 50,
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
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 53,
                                                            end: 59,
                                                            as_str(): "second",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 59,
                                                            end: 60,
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
                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                        ),
                                                                        start: 61,
                                                                        end: 62,
                                                                        as_str(): "F",
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
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 37,
                                        end: 64,
                                        as_str(): "{\n  first: T,\n  second: F\n}",
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
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 68,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 69,
                                            end: 84,
                                            as_str(): "double_identity",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f8c2c380,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                        ),
                                                        start: 84,
                                                        end: 85,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 85,
                                                                    end: 86,
                                                                    as_str(): "T",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            CommaToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 86,
                                                                    end: 87,
                                                                    as_str(): ",",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 88,
                                                                end: 89,
                                                                as_str(): "F",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f8c2c380,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                        ),
                                                        start: 89,
                                                        end: 90,
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
                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                        ),
                                                                        start: 91,
                                                                        end: 92,
                                                                        as_str(): "x",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 92,
                                                                    end: 93,
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
                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                ),
                                                                                start: 94,
                                                                                end: 95,
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
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 95,
                                                                end: 96,
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
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 97,
                                                                    end: 98,
                                                                    as_str(): "y",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
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
                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                            ),
                                                                            start: 100,
                                                                            end: 101,
                                                                            as_str(): "F",
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
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 90,
                                            end: 102,
                                            as_str(): "(x: T, y: F)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 103,
                                                    end: 105,
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
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 106,
                                                                end: 120,
                                                                as_str(): "DoubleIdentity",
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
                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                ),
                                                                                start: 120,
                                                                                end: 121,
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
                                                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 121,
                                                                                                        end: 122,
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
                                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 122,
                                                                                            end: 123,
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
                                                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 124,
                                                                                                    end: 125,
                                                                                                    as_str(): "F",
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
                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
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
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 131,
                                                                end: 145,
                                                                as_str(): "DoubleIdentity",
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
                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                            ),
                                                                            start: 152,
                                                                            end: 157,
                                                                            as_str(): "first",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    expr_opt: Some(
                                                                        (
                                                                            ColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 157,
                                                                                    end: 158,
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
                                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 159,
                                                                                                end: 160,
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
                                                                    ),
                                                                },
                                                                CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                        ),
                                                                        start: 160,
                                                                        end: 161,
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
                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                        ),
                                                                        start: 166,
                                                                        end: 172,
                                                                        as_str(): "second",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                expr_opt: Some(
                                                                    (
                                                                        ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                ),
                                                                                start: 172,
                                                                                end: 173,
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
                                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 174,
                                                                                            end: 175,
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
                                                        src (ptr): 0x00007fe0f8c2c380,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                        ),
                                                        start: 146,
                                                        end: 179,
                                                        as_str(): "{\n    first: x,\n    second: y\n  }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 127,
                                        end: 181,
                                        as_str(): "{\n  DoubleIdentity {\n    first: x,\n    second: y\n  }\n}",
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
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 183,
                                            end: 185,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 186,
                                            end: 190,
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
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 190,
                                            end: 192,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 193,
                                                    end: 195,
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
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 196,
                                                                end: 200,
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
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 205,
                                                            end: 208,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 209,
                                                                end: 217,
                                                                as_str(): "double_a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 218,
                                                            end: 219,
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
                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                            ),
                                                                            start: 220,
                                                                            end: 235,
                                                                            as_str(): "double_identity",
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
                                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 236,
                                                                                        end: 240,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                    kind: True,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                ),
                                                                                start: 240,
                                                                                end: 241,
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
                                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 242,
                                                                                    end: 246,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 235,
                                                                end: 247,
                                                                as_str(): "(true, true)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 247,
                                                            end: 248,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 251,
                                                            end: 254,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 255,
                                                                end: 263,
                                                                as_str(): "double_b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 264,
                                                            end: 265,
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
                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                            ),
                                                                            start: 266,
                                                                            end: 281,
                                                                            as_str(): "double_identity",
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
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 282,
                                                                                        end: 284,
                                                                                        as_str(): "10",
                                                                                    },
                                                                                    parsed: 10,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U32,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 284,
                                                                                                end: 287,
                                                                                                as_str(): "u32",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                ),
                                                                                start: 287,
                                                                                end: 288,
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
                                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 289,
                                                                                    end: 291,
                                                                                    as_str(): "43",
                                                                                },
                                                                                parsed: 43,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 291,
                                                                                            end: 294,
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
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 281,
                                                                end: 295,
                                                                as_str(): "(10u32, 43u64)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
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
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
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
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 333,
                                                                end: 341,
                                                                as_str(): "double_a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 341,
                                                                    end: 342,
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
                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                ),
                                                                                start: 343,
                                                                                end: 357,
                                                                                as_str(): "DoubleIdentity",
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
                                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 357,
                                                                                                end: 358,
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
                                                                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 358,
                                                                                                                        end: 362,
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
                                                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 362,
                                                                                                            end: 363,
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
                                                                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 364,
                                                                                                                    end: 368,
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
                                                                                        },
                                                                                        close_angle_bracket_token: CloseAngleBracketToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 368,
                                                                                                end: 369,
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
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 370,
                                                            end: 371,
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
                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                            ),
                                                                            start: 372,
                                                                            end: 387,
                                                                            as_str(): "double_identity",
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
                                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 388,
                                                                                        end: 392,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                    kind: True,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                ),
                                                                                start: 392,
                                                                                end: 393,
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
                                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 394,
                                                                                    end: 398,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 387,
                                                                end: 399,
                                                                as_str(): "(true, true)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 399,
                                                            end: 400,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 403,
                                                            end: 406,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 407,
                                                                end: 415,
                                                                as_str(): "double_b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 415,
                                                                    end: 416,
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
                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                ),
                                                                                start: 417,
                                                                                end: 431,
                                                                                as_str(): "DoubleIdentity",
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
                                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 431,
                                                                                                end: 432,
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
                                                                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 432,
                                                                                                                        end: 435,
                                                                                                                        as_str(): "u32",
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
                                                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 435,
                                                                                                            end: 436,
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
                                                                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 437,
                                                                                                                    end: 440,
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
                                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 440,
                                                                                                end: 441,
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
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 442,
                                                            end: 443,
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
                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                            ),
                                                                            start: 444,
                                                                            end: 459,
                                                                            as_str(): "double_identity",
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
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 460,
                                                                                        end: 462,
                                                                                        as_str(): "10",
                                                                                    },
                                                                                    parsed: 10,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U32,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 462,
                                                                                                end: 465,
                                                                                                as_str(): "u32",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                ),
                                                                                start: 465,
                                                                                end: 466,
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
                                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 467,
                                                                                    end: 469,
                                                                                    as_str(): "43",
                                                                                },
                                                                                parsed: 43,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe0f8c2c380,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 469,
                                                                                            end: 472,
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
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 459,
                                                                end: 473,
                                                                as_str(): "(10u32, 43u64)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 473,
                                                            end: 474,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            FieldProjection {
                                                target: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 478,
                                                                    end: 486,
                                                                    as_str(): "double_a",
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
                                                        src (ptr): 0x00007fe0f8c2c380,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                        ),
                                                        start: 486,
                                                        end: 487,
                                                        as_str(): ".",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f8c2c380,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                        ),
                                                        start: 487,
                                                        end: 492,
                                                        as_str(): "first",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 201,
                                        end: 494,
                                        as_str(): "{\n  let double_a = double_identity(true, true);\n  let double_b = double_identity(10u32, 43u64);\n\n  // for testing annotations\n  let double_a: DoubleIdentity<bool, bool> = double_identity(true, true);\n  let double_b: DoubleIdentity<u32, u64> = double_identity(10u32, 43u64);\n\n  double_a.first\n}",
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
