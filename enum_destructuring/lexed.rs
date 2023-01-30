Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb10d4510c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb10d4510c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
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
                                        src (ptr): 0x00007fb10d4510c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 13,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb10d4510c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                        ),
                                        start: 14,
                                        end: 20,
                                        as_str(): "Result",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 20,
                                                    end: 21,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 21,
                                                                end: 22,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 22,
                                                                end: 23,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb10d4510c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                            ),
                                                            start: 24,
                                                            end: 25,
                                                            as_str(): "E",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 25,
                                                    end: 26,
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
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 31,
                                                                end: 33,
                                                                as_str(): "Ok",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 33,
                                                                end: 34,
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
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 35,
                                                                            end: 36,
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
                                                        src (ptr): 0x00007fb10d4510c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                        ),
                                                        start: 36,
                                                        end: 37,
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
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 40,
                                                                end: 43,
                                                                as_str(): "Err",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 43,
                                                                end: 44,
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
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 45,
                                                                            end: 46,
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
                                                        src (ptr): 0x00007fb10d4510c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                        ),
                                                        start: 46,
                                                        end: 47,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb10d4510c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                        ),
                                        start: 27,
                                        end: 49,
                                        as_str(): "{\n  Ok: T,\n  Err: E,\n}",
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
                                            src (ptr): 0x00007fb10d4510c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                            ),
                                            start: 51,
                                            end: 53,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb10d4510c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                            ),
                                            start: 54,
                                            end: 58,
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
                                            src (ptr): 0x00007fb10d4510c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                            ),
                                            start: 58,
                                            end: 60,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 61,
                                                    end: 63,
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
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 64,
                                                                end: 67,
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
                                                            src (ptr): 0x00007fb10d4510c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                            ),
                                                            start: 73,
                                                            end: 76,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 77,
                                                                end: 78,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 78,
                                                                    end: 79,
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
                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 80,
                                                                                end: 86,
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
                                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 86,
                                                                                                end: 87,
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
                                                                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 87,
                                                                                                                        end: 90,
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
                                                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 90,
                                                                                                            end: 91,
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
                                                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
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
                                                                                            ),
                                                                                        },
                                                                                        close_angle_bracket_token: CloseAngleBracketToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 95,
                                                                                                end: 96,
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
                                                            src (ptr): 0x00007fb10d4510c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                            ),
                                                            start: 97,
                                                            end: 98,
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
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 99,
                                                                            end: 105,
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
                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 105,
                                                                                end: 107,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 107,
                                                                                    end: 109,
                                                                                    as_str(): "Ok",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: Some(
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 109,
                                                                                            end: 111,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    GenericArgs {
                                                                                        parameters: AngleBrackets {
                                                                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 111,
                                                                                                    end: 112,
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
                                                                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 112,
                                                                                                                            end: 115,
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
                                                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
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
                                                                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 117,
                                                                                                                        end: 120,
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
                                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 120,
                                                                                                    end: 121,
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
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 122,
                                                                                    end: 123,
                                                                                    as_str(): "5",
                                                                                },
                                                                                parsed: 5,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 123,
                                                                                            end: 126,
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
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 121,
                                                                end: 127,
                                                                as_str(): "(5u64)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb10d4510c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                            ),
                                                            start: 127,
                                                            end: 128,
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
                                                            src (ptr): 0x00007fb10d4510c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                            ),
                                                            start: 158,
                                                            end: 160,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Let {
                                                        let_token: LetToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 161,
                                                                end: 164,
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
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 165,
                                                                            end: 171,
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
                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 171,
                                                                                end: 173,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 173,
                                                                                    end: 175,
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
                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 176,
                                                                                    end: 177,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 175,
                                                                    end: 178,
                                                                    as_str(): "(y)",
                                                                },
                                                            },
                                                        },
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 179,
                                                                end: 180,
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
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 181,
                                                                            end: 182,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                        ),
                                                                                        start: 185,
                                                                                        end: 186,
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
                                                                    add_token: AddToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 187,
                                                                            end: 188,
                                                                            as_str(): "+",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 189,
                                                                                    end: 191,
                                                                                    as_str(): "10",
                                                                                },
                                                                                parsed: 10,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb10d4510c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                            ),
                                                            start: 183,
                                                            end: 193,
                                                            as_str(): "{ y + 10 }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 194,
                                                                    end: 198,
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
                                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 201,
                                                                                            end: 202,
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
                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 199,
                                                                        end: 204,
                                                                        as_str(): "{ 1 }",
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
                                        src (ptr): 0x00007fb10d4510c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                        ),
                                        start: 68,
                                        end: 206,
                                        as_str(): "{\n   let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);\n\n    // should return 15\n    if let Result::Ok(y) = x { y + 10 } else { 1 }\n}",
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
