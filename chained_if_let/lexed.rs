Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb12e8db920,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb12e8db920,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                        src (ptr): 0x00007fb12e8db920,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 13,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb12e8db920,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 21,
                                                                end: 22,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 31,
                                                                end: 33,
                                                                as_str(): "Ok",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                        src (ptr): 0x00007fb12e8db920,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 40,
                                                                end: 43,
                                                                as_str(): "Err",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                        src (ptr): 0x00007fb12e8db920,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                        src (ptr): 0x00007fb12e8db920,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                            src (ptr): 0x00007fb12e8db920,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                            ),
                                            start: 70,
                                            end: 72,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb12e8db920,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                            ),
                                            start: 73,
                                            end: 77,
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
                                            src (ptr): 0x00007fb12e8db920,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                            ),
                                            start: 77,
                                            end: 79,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 80,
                                                    end: 82,
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
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 83,
                                                                end: 86,
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
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                            ),
                                                            start: 91,
                                                            end: 94,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 95,
                                                                end: 103,
                                                                as_str(): "result_a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                            ),
                                                            start: 104,
                                                            end: 105,
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
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 106,
                                                                            end: 112,
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
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 112,
                                                                                end: 114,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 114,
                                                                                    end: 116,
                                                                                    as_str(): "Ok",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: Some(
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 116,
                                                                                            end: 118,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    GenericArgs {
                                                                                        parameters: AngleBrackets {
                                                                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 118,
                                                                                                    end: 119,
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
                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 119,
                                                                                                                            end: 122,
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
                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 124,
                                                                                                                        end: 128,
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
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 128,
                                                                                                    end: 129,
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
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 130,
                                                                                    end: 131,
                                                                                    as_str(): "5",
                                                                                },
                                                                                parsed: 5,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 131,
                                                                                            end: 134,
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
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 129,
                                                                end: 135,
                                                                as_str(): "(5u64)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 136,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                            ),
                                                            start: 139,
                                                            end: 142,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 143,
                                                                end: 151,
                                                                as_str(): "result_b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                            ),
                                                            start: 152,
                                                            end: 153,
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
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 154,
                                                                            end: 160,
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
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 160,
                                                                                end: 162,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 162,
                                                                                    end: 165,
                                                                                    as_str(): "Err",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: Some(
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 165,
                                                                                            end: 167,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    GenericArgs {
                                                                                        parameters: AngleBrackets {
                                                                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 167,
                                                                                                    end: 168,
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
                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 168,
                                                                                                                            end: 171,
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
                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 171,
                                                                                                                end: 172,
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
                                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 173,
                                                                                                                        end: 177,
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
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 177,
                                                                                                    end: 178,
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
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 179,
                                                                                    end: 184,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 178,
                                                                end: 185,
                                                                as_str(): "(false)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                            ),
                                                            start: 185,
                                                            end: 186,
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
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                            ),
                                                            start: 190,
                                                            end: 192,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Let {
                                                        let_token: LetToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 193,
                                                                end: 196,
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
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 197,
                                                                            end: 203,
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
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 203,
                                                                                end: 205,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 205,
                                                                                    end: 208,
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
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 209,
                                                                                    end: 210,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12e8db920,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                    ),
                                                                    start: 208,
                                                                    end: 211,
                                                                    as_str(): "(a)",
                                                                },
                                                            },
                                                        },
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 212,
                                                                end: 213,
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
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 214,
                                                                            end: 222,
                                                                            as_str(): "result_a",
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
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 229,
                                                                                end: 230,
                                                                                as_str(): "6",
                                                                            },
                                                                            parsed: 6,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                            ),
                                                            start: 223,
                                                            end: 234,
                                                            as_str(): "{\n    6\n  }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12e8db920,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                    ),
                                                                    start: 235,
                                                                    end: 239,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Continue(
                                                                IfExpr {
                                                                    if_token: IfToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 240,
                                                                            end: 242,
                                                                            as_str(): "if",
                                                                        },
                                                                    },
                                                                    condition: Let {
                                                                        let_token: LetToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 243,
                                                                                end: 246,
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
                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 247,
                                                                                            end: 253,
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
                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 253,
                                                                                                end: 255,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 255,
                                                                                                    end: 257,
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
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 258,
                                                                                                    end: 261,
                                                                                                    as_str(): "num",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 257,
                                                                                    end: 262,
                                                                                    as_str(): "(num)",
                                                                                },
                                                                            },
                                                                        },
                                                                        eq_token: EqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 263,
                                                                                end: 264,
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
                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 265,
                                                                                            end: 273,
                                                                                            as_str(): "result_b",
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
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 280,
                                                                                                end: 282,
                                                                                                as_str(): "10",
                                                                                            },
                                                                                            parsed: 10,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 274,
                                                                            end: 286,
                                                                            as_str(): "{\n    10\n  }",
                                                                        },
                                                                    },
                                                                    else_opt: Some(
                                                                        (
                                                                            ElseToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 287,
                                                                                    end: 291,
                                                                                    as_str(): "else",
                                                                                },
                                                                            },
                                                                            Continue(
                                                                                IfExpr {
                                                                                    if_token: IfToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 292,
                                                                                            end: 294,
                                                                                            as_str(): "if",
                                                                                        },
                                                                                    },
                                                                                    condition: Let {
                                                                                        let_token: LetToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 295,
                                                                                                end: 298,
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
                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 299,
                                                                                                            end: 305,
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
                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 305,
                                                                                                                end: 307,
                                                                                                                as_str(): "::",
                                                                                                            },
                                                                                                        },
                                                                                                        PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 307,
                                                                                                                    end: 309,
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
                                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 310,
                                                                                                                    end: 313,
                                                                                                                    as_str(): "num",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 309,
                                                                                                    end: 314,
                                                                                                    as_str(): "(num)",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                        eq_token: EqToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 315,
                                                                                                end: 316,
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
                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 317,
                                                                                                            end: 325,
                                                                                                            as_str(): "result_a",
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
                                                                                                Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 332,
                                                                                                                    end: 335,
                                                                                                                    as_str(): "num",
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
                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 326,
                                                                                            end: 339,
                                                                                            as_str(): "{\n    num\n  }",
                                                                                        },
                                                                                    },
                                                                                    else_opt: Some(
                                                                                        (
                                                                                            ElseToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 340,
                                                                                                    end: 344,
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
                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 352,
                                                                                                                            end: 354,
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
                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 345,
                                                                                                        end: 359,
                                                                                                        as_str(): "{ \n    42 \n  }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb12e8db920,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                        ),
                                        start: 87,
                                        end: 361,
                                        as_str(): "{\n  let result_a = Result::Ok::<u64, bool>(5u64);\n  let result_b = Result::Err::<u64, bool>(false);\n\n  if let Result::Err(a) = result_a {\n    6\n  } else if let Result::Ok(num) = result_b {\n    10\n  } else if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }\n}",
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
