Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe056083bb0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe056083bb0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 15,
                                            as_str(): "smo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 15,
                                                        end: 16,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 16,
                                                                end: 17,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 17,
                                                        end: 18,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 19,
                                                                        end: 28,
                                                                        as_str(): "recipient",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 28,
                                                                    end: 29,
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
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 30,
                                                                                end: 34,
                                                                                as_str(): "b256",
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 34,
                                                                end: 35,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 36,
                                                                        end: 41,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 41,
                                                                    end: 42,
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
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 43,
                                                                                end: 44,
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 44,
                                                                end: 45,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 46,
                                                                        end: 58,
                                                                        as_str(): "output_index",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 58,
                                                                    end: 59,
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
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 60,
                                                                                end: 63,
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
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 63,
                                                                end: 64,
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
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 65,
                                                                    end: 70,
                                                                    as_str(): "coins",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 70,
                                                                end: 71,
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
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 72,
                                                                            end: 75,
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
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 18,
                                            end: 76,
                                            as_str(): "(recipient: b256, value: T, output_index: u64, coins: u64)",
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 83,
                                                                        end: 88,
                                                                        as_str(): "__smo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: Some(
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 88,
                                                                                end: 90,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        GenericArgs {
                                                                            parameters: AngleBrackets {
                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 90,
                                                                                        end: 91,
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
                                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                            ),
                                                                                                            start: 91,
                                                                                                            end: 92,
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
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 92,
                                                                                        end: 93,
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
                                                            value_separator_pairs: [
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 94,
                                                                                        end: 103,
                                                                                        as_str(): "recipient",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 103,
                                                                            end: 104,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 105,
                                                                                        end: 110,
                                                                                        as_str(): "value",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 110,
                                                                            end: 111,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 112,
                                                                                        end: 124,
                                                                                        as_str(): "output_index",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 124,
                                                                            end: 125,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 126,
                                                                                    end: 131,
                                                                                    as_str(): "coins",
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 93,
                                                            end: 132,
                                                            as_str(): "(recipient, value, output_index, coins)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 132,
                                                            end: 133,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 77,
                                        end: 135,
                                        as_str(): "{\n    __smo::<T>(recipient, value, output_index, coins);\n}",
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
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 137,
                                        end: 143,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 144,
                                        end: 154,
                                        as_str(): "TestStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 154,
                                                    end: 155,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 155,
                                                            end: 156,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 156,
                                                    end: 157,
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 164,
                                                                end: 171,
                                                                as_str(): "field_1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 171,
                                                                end: 172,
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
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
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
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 177,
                                                        end: 178,
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 183,
                                                                end: 190,
                                                                as_str(): "field_2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 190,
                                                                end: 191,
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
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 192,
                                                                            end: 193,
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
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 193,
                                                        end: 194,
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 199,
                                                                end: 206,
                                                                as_str(): "field_3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 206,
                                                                end: 207,
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
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 208,
                                                                            end: 211,
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
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 211,
                                                        end: 212,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 158,
                                        end: 214,
                                        as_str(): "{\n    field_1: bool,\n    field_2: T,\n    field_3: u64,\n}",
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
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 216,
                                        end: 220,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 221,
                                        end: 229,
                                        as_str(): "TestEnum",
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 236,
                                                                end: 246,
                                                                as_str(): "VariantOne",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 246,
                                                                end: 247,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 248,
                                                                    end: 250,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 250,
                                                        end: 251,
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 256,
                                                                end: 266,
                                                                as_str(): "VariantTwo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 266,
                                                                end: 267,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 268,
                                                                    end: 270,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 270,
                                                        end: 271,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 230,
                                        end: 273,
                                        as_str(): "{\n    VariantOne: (),\n    VariantTwo: (),\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Enum(
                            ItemEnum {
                                visibility: Some(
                                    PubToken {
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 275,
                                            end: 278,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 279,
                                        end: 283,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 284,
                                        end: 290,
                                        as_str(): "Option",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 290,
                                                    end: 291,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 291,
                                                            end: 292,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 292,
                                                    end: 293,
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 300,
                                                                end: 304,
                                                                as_str(): "None",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 304,
                                                                end: 305,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 306,
                                                                    end: 308,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 308,
                                                        end: 309,
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 314,
                                                                end: 318,
                                                                as_str(): "Some",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 318,
                                                                end: 319,
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
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 320,
                                                                            end: 321,
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
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 321,
                                                        end: 322,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 294,
                                        end: 324,
                                        as_str(): "{\n    None: (),\n    Some: T,\n}",
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
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 326,
                                            end: 328,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 329,
                                            end: 333,
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
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 333,
                                            end: 335,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 336,
                                                    end: 338,
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 339,
                                                                end: 343,
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 350,
                                                            end: 353,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 354,
                                                                end: 363,
                                                                as_str(): "recipient",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 364,
                                                            end: 365,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 366,
                                                                    end: 432,
                                                                    as_str(): "0x0101010101010101010101010101010101010101010101010101010101010101",
                                                                },
                                                                parsed: 454086624460063511464984254936031011189294057512315937409637584344757371137,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 432,
                                                            end: 433,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 438,
                                                            end: 441,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 442,
                                                                end: 454,
                                                                as_str(): "output_index",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 455,
                                                            end: 456,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 457,
                                                                    end: 458,
                                                                    as_str(): "3",
                                                                },
                                                                parsed: 3,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 458,
                                                            end: 459,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 464,
                                                            end: 467,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 468,
                                                                end: 473,
                                                                as_str(): "coins",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 474,
                                                            end: 475,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 476,
                                                                    end: 478,
                                                                    as_str(): "24",
                                                                },
                                                                parsed: 24,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 478,
                                                            end: 479,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 544,
                                                            end: 547,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 548,
                                                                end: 549,
                                                                as_str(): "k",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 549,
                                                                    end: 550,
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
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 551,
                                                                                end: 555,
                                                                                as_str(): "b256",
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 556,
                                                            end: 557,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 558,
                                                                    end: 624,
                                                                    as_str(): "0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a",
                                                                },
                                                                parsed: 108340740691025251715289040289762951404984910512315932338997285918435378372170,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 624,
                                                            end: 625,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 630,
                                                            end: 633,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 634,
                                                                end: 635,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 635,
                                                                    end: 636,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Str {
                                                                str_token: StrToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 637,
                                                                        end: 640,
                                                                        as_str(): "str",
                                                                    },
                                                                },
                                                                length: SquareBrackets {
                                                                    inner: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 641,
                                                                                    end: 642,
                                                                                    as_str(): "4",
                                                                                },
                                                                                parsed: 4,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 640,
                                                                        end: 643,
                                                                        as_str(): "[4]",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 644,
                                                            end: 645,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        String(
                                                            LitString {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 646,
                                                                    end: 652,
                                                                    as_str(): "\"Fuel\"",
                                                                },
                                                                parsed: "Fuel",
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 652,
                                                            end: 653,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 658,
                                                            end: 661,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 662,
                                                                end: 663,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 663,
                                                                    end: 664,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Array(
                                                                SquareBrackets {
                                                                    inner: TyArrayDescriptor {
                                                                        ty: Path(
                                                                            PathType {
                                                                                root_opt: None,
                                                                                prefix: PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
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
                                                                        semicolon_token: SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 668,
                                                                                end: 669,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 670,
                                                                                        end: 671,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                    parsed: 3,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 665,
                                                                        end: 672,
                                                                        as_str(): "[u8; 3]",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 673,
                                                            end: 674,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
                                                                    value_separator_pairs: [
                                                                        (
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                            ),
                                                                                            start: 676,
                                                                                            end: 677,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: Some(
                                                                                            (
                                                                                                U8,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                    ),
                                                                                                    start: 677,
                                                                                                    end: 679,
                                                                                                    as_str(): "u8",
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 679,
                                                                                    end: 680,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                            ),
                                                                                            start: 681,
                                                                                            end: 682,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                        parsed: 2,
                                                                                        ty_opt: Some(
                                                                                            (
                                                                                                U8,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                    ),
                                                                                                    start: 682,
                                                                                                    end: 684,
                                                                                                    as_str(): "u8",
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 684,
                                                                                    end: 685,
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
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 686,
                                                                                        end: 687,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                    parsed: 3,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U8,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                ),
                                                                                                start: 687,
                                                                                                end: 689,
                                                                                                as_str(): "u8",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 675,
                                                                end: 690,
                                                                as_str(): "[1u8, 2u8, 3u8]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 690,
                                                            end: 691,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 696,
                                                            end: 699,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 700,
                                                                end: 711,
                                                                as_str(): "test_struct",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 712,
                                                            end: 713,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 714,
                                                                        end: 724,
                                                                        as_str(): "TestStruct",
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
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 735,
                                                                                    end: 742,
                                                                                    as_str(): "field_1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                            ),
                                                                                            start: 742,
                                                                                            end: 743,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                    ),
                                                                                                    start: 744,
                                                                                                    end: 748,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 748,
                                                                                end: 749,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 758,
                                                                                    end: 765,
                                                                                    as_str(): "field_2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                            ),
                                                                                            start: 765,
                                                                                            end: 766,
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
                                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                        ),
                                                                                                        start: 767,
                                                                                                        end: 768,
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
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 768,
                                                                                end: 769,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 778,
                                                                                    end: 785,
                                                                                    as_str(): "field_3",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                            ),
                                                                                            start: 785,
                                                                                            end: 786,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                    ),
                                                                                                    start: 787,
                                                                                                    end: 789,
                                                                                                    as_str(): "11",
                                                                                                },
                                                                                                parsed: 11,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 789,
                                                                                end: 790,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 725,
                                                                end: 796,
                                                                as_str(): "{\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 796,
                                                            end: 797,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 803,
                                                            end: 806,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 807,
                                                                end: 816,
                                                                as_str(): "test_enum",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 817,
                                                            end: 818,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 819,
                                                                        end: 827,
                                                                        as_str(): "TestEnum",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 827,
                                                                            end: 829,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 829,
                                                                                end: 839,
                                                                                as_str(): "VariantTwo",
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
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 839,
                                                            end: 840,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 845,
                                                                        end: 848,
                                                                        as_str(): "smo",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 849,
                                                                                        end: 858,
                                                                                        as_str(): "recipient",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 858,
                                                                            end: 859,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 860,
                                                                                        end: 861,
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
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 861,
                                                                            end: 862,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 863,
                                                                                        end: 875,
                                                                                        as_str(): "output_index",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 875,
                                                                            end: 876,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 877,
                                                                                    end: 882,
                                                                                    as_str(): "coins",
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 848,
                                                            end: 883,
                                                            as_str(): "(recipient, k, output_index, coins)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 883,
                                                            end: 884,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 889,
                                                                        end: 892,
                                                                        as_str(): "smo",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 893,
                                                                                        end: 902,
                                                                                        as_str(): "recipient",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 902,
                                                                            end: 903,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 904,
                                                                                    end: 906,
                                                                                    as_str(): "42",
                                                                                },
                                                                                parsed: 42,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 906,
                                                                            end: 907,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 908,
                                                                                        end: 920,
                                                                                        as_str(): "output_index",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 920,
                                                                            end: 921,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 922,
                                                                                    end: 927,
                                                                                    as_str(): "coins",
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 892,
                                                            end: 928,
                                                            as_str(): "(recipient, 42, output_index, coins)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 928,
                                                            end: 929,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 934,
                                                                        end: 937,
                                                                        as_str(): "smo",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 938,
                                                                                        end: 947,
                                                                                        as_str(): "recipient",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 947,
                                                                            end: 948,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 949,
                                                                                    end: 951,
                                                                                    as_str(): "42",
                                                                                },
                                                                                parsed: 42,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                            ),
                                                                                            start: 951,
                                                                                            end: 954,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 954,
                                                                            end: 955,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 956,
                                                                                        end: 968,
                                                                                        as_str(): "output_index",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 968,
                                                                            end: 969,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 970,
                                                                                    end: 975,
                                                                                    as_str(): "coins",
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 937,
                                                            end: 976,
                                                            as_str(): "(recipient, 42u32, output_index, coins)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 976,
                                                            end: 977,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 982,
                                                                        end: 985,
                                                                        as_str(): "smo",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 986,
                                                                                        end: 995,
                                                                                        as_str(): "recipient",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 995,
                                                                            end: 996,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 997,
                                                                                    end: 999,
                                                                                    as_str(): "42",
                                                                                },
                                                                                parsed: 42,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U16,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                            ),
                                                                                            start: 999,
                                                                                            end: 1002,
                                                                                            as_str(): "u16",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1002,
                                                                            end: 1003,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1004,
                                                                                        end: 1016,
                                                                                        as_str(): "output_index",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1016,
                                                                            end: 1017,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 1018,
                                                                                    end: 1023,
                                                                                    as_str(): "coins",
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 985,
                                                            end: 1024,
                                                            as_str(): "(recipient, 42u16, output_index, coins)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1024,
                                                            end: 1025,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 1030,
                                                                        end: 1033,
                                                                        as_str(): "smo",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1034,
                                                                                        end: 1043,
                                                                                        as_str(): "recipient",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1043,
                                                                            end: 1044,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 1045,
                                                                                    end: 1047,
                                                                                    as_str(): "42",
                                                                                },
                                                                                parsed: 42,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                            ),
                                                                                            start: 1047,
                                                                                            end: 1049,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1049,
                                                                            end: 1050,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1051,
                                                                                        end: 1063,
                                                                                        as_str(): "output_index",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1063,
                                                                            end: 1064,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 1065,
                                                                                    end: 1070,
                                                                                    as_str(): "coins",
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1033,
                                                            end: 1071,
                                                            as_str(): "(recipient, 42u8, output_index, coins)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1071,
                                                            end: 1072,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 1077,
                                                                        end: 1082,
                                                                        as_str(): "__smo",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1083,
                                                                                        end: 1092,
                                                                                        as_str(): "recipient",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1092,
                                                                            end: 1093,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1094,
                                                                                        end: 1095,
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
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1095,
                                                                            end: 1096,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1097,
                                                                                        end: 1109,
                                                                                        as_str(): "output_index",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1109,
                                                                            end: 1110,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 1111,
                                                                                    end: 1116,
                                                                                    as_str(): "coins",
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1082,
                                                            end: 1117,
                                                            as_str(): "(recipient, a, output_index, coins)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1117,
                                                            end: 1118,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 1123,
                                                                        end: 1128,
                                                                        as_str(): "__smo",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1129,
                                                                                        end: 1138,
                                                                                        as_str(): "recipient",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1138,
                                                                            end: 1139,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1140,
                                                                                        end: 1141,
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
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1141,
                                                                            end: 1142,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1143,
                                                                                        end: 1155,
                                                                                        as_str(): "output_index",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1155,
                                                                            end: 1156,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 1157,
                                                                                    end: 1162,
                                                                                    as_str(): "coins",
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1128,
                                                            end: 1163,
                                                            as_str(): "(recipient, b, output_index, coins)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1163,
                                                            end: 1164,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 1169,
                                                                        end: 1174,
                                                                        as_str(): "__smo",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1175,
                                                                                        end: 1184,
                                                                                        as_str(): "recipient",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1184,
                                                                            end: 1185,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1186,
                                                                                        end: 1197,
                                                                                        as_str(): "test_struct",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1197,
                                                                            end: 1198,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1199,
                                                                                        end: 1211,
                                                                                        as_str(): "output_index",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1211,
                                                                            end: 1212,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 1213,
                                                                                    end: 1218,
                                                                                    as_str(): "coins",
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1174,
                                                            end: 1219,
                                                            as_str(): "(recipient, test_struct, output_index, coins)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1219,
                                                            end: 1220,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 1225,
                                                                        end: 1230,
                                                                        as_str(): "__smo",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1231,
                                                                                        end: 1240,
                                                                                        as_str(): "recipient",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1240,
                                                                            end: 1241,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1242,
                                                                                        end: 1251,
                                                                                        as_str(): "test_enum",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1251,
                                                                            end: 1252,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1253,
                                                                                        end: 1265,
                                                                                        as_str(): "output_index",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1265,
                                                                            end: 1266,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 1267,
                                                                                    end: 1272,
                                                                                    as_str(): "coins",
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1230,
                                                            end: 1273,
                                                            as_str(): "(recipient, test_enum, output_index, coins)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1273,
                                                            end: 1274,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 1279,
                                                                        end: 1284,
                                                                        as_str(): "__smo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: Some(
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 1284,
                                                                                end: 1286,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        GenericArgs {
                                                                            parameters: AngleBrackets {
                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1286,
                                                                                        end: 1287,
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
                                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1287,
                                                                                                            end: 1293,
                                                                                                            as_str(): "Option",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    generics_opt: Some(
                                                                                                        (
                                                                                                            Some(
                                                                                                                DoubleColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1293,
                                                                                                                        end: 1295,
                                                                                                                        as_str(): "::",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            GenericArgs {
                                                                                                                parameters: AngleBrackets {
                                                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1295,
                                                                                                                            end: 1296,
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
                                                                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1296,
                                                                                                                                                end: 1306,
                                                                                                                                                as_str(): "TestStruct",
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
                                                                                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 1306,
                                                                                                                                                                end: 1307,
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
                                                                                                                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1307,
                                                                                                                                                                                    end: 1310,
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
                                                                                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 1310,
                                                                                                                                                                end: 1311,
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
                                                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1311,
                                                                                                                            end: 1312,
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
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1312,
                                                                                        end: 1313,
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
                                                            value_separator_pairs: [
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1314,
                                                                                        end: 1323,
                                                                                        as_str(): "recipient",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1323,
                                                                            end: 1324,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                            ),
                                                                                            start: 1325,
                                                                                            end: 1331,
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
                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                ),
                                                                                                start: 1331,
                                                                                                end: 1333,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1333,
                                                                                                    end: 1337,
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
                                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1338,
                                                                                                        end: 1348,
                                                                                                        as_str(): "TestStruct",
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
                                                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1359,
                                                                                                                    end: 1366,
                                                                                                                    as_str(): "field_1",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            expr_opt: Some(
                                                                                                                (
                                                                                                                    ColonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1366,
                                                                                                                            end: 1367,
                                                                                                                            as_str(): ":",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    Literal(
                                                                                                                        Bool(
                                                                                                                            LitBool {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1368,
                                                                                                                                    end: 1372,
                                                                                                                                    as_str(): "true",
                                                                                                                                },
                                                                                                                                kind: True,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1372,
                                                                                                                end: 1373,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    (
                                                                                                        ExprStructField {
                                                                                                            field_name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1382,
                                                                                                                    end: 1389,
                                                                                                                    as_str(): "field_2",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            expr_opt: Some(
                                                                                                                (
                                                                                                                    ColonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1389,
                                                                                                                            end: 1390,
                                                                                                                            as_str(): ":",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    Literal(
                                                                                                                        Int(
                                                                                                                            LitInt {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1391,
                                                                                                                                    end: 1393,
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
                                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1393,
                                                                                                                end: 1394,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    (
                                                                                                        ExprStructField {
                                                                                                            field_name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1403,
                                                                                                                    end: 1410,
                                                                                                                    as_str(): "field_3",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            expr_opt: Some(
                                                                                                                (
                                                                                                                    ColonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1410,
                                                                                                                            end: 1411,
                                                                                                                            as_str(): ":",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    Literal(
                                                                                                                        Int(
                                                                                                                            LitInt {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1412,
                                                                                                                                    end: 1414,
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
                                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1414,
                                                                                                                end: 1415,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                                final_value_opt: None,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                ),
                                                                                                start: 1349,
                                                                                                end: 1421,
                                                                                                as_str(): "{\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 1337,
                                                                                end: 1422,
                                                                                as_str(): "(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    })",
                                                                            },
                                                                        },
                                                                    },
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1422,
                                                                            end: 1423,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1424,
                                                                                        end: 1436,
                                                                                        as_str(): "output_index",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1436,
                                                                            end: 1437,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 1438,
                                                                                    end: 1443,
                                                                                    as_str(): "coins",
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1313,
                                                            end: 1444,
                                                            as_str(): "(recipient, Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }), output_index, coins)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1444,
                                                            end: 1445,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 1517,
                                                                        end: 1522,
                                                                        as_str(): "__log",
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
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 1523,
                                                                                    end: 1524,
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1522,
                                                            end: 1525,
                                                            as_str(): "(a)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1525,
                                                            end: 1526,
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 1531,
                                                                        end: 1536,
                                                                        as_str(): "__log",
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
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 1537,
                                                                                    end: 1538,
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1536,
                                                            end: 1539,
                                                            as_str(): "(b)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1539,
                                                            end: 1540,
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
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1546,
                                                            end: 1550,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 344,
                                        end: 1552,
                                        as_str(): "{\n    let recipient = 0x0101010101010101010101010101010101010101010101010101010101010101;\n    let output_index = 3;\n    let coins = 24;\n\n    // Check various data types as message data in `__smo`\n    let k: b256 = 0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a;\n    let a: str[4] = \"Fuel\";\n    let b: [u8; 3] = [1u8, 2u8, 3u8];\n    let test_struct = TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    };\n\n    let test_enum = TestEnum::VariantTwo;\n    smo(recipient, k, output_index, coins);\n    smo(recipient, 42, output_index, coins);\n    smo(recipient, 42u32, output_index, coins);\n    smo(recipient, 42u16, output_index, coins);\n    smo(recipient, 42u8, output_index, coins);\n    __smo(recipient, a, output_index, coins);\n    __smo(recipient, b, output_index, coins);\n    __smo(recipient, test_struct, output_index, coins);\n    __smo(recipient, test_enum, output_index, coins);\n    __smo::<Option::<TestStruct<u64>>>(recipient, Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }), output_index, coins);\n\n    // Make sure that logs don't clobber messages in the JSON ABI\n    __log(a);\n    __log(b);\n\n    true\n}",
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
