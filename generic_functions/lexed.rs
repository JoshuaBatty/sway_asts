Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0fc097840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
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
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 20,
                                            as_str(): "identity",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc097840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                        ),
                                                        start: 20,
                                                        end: 21,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 21,
                                                                end: 22,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc097840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                        ),
                                                        start: 22,
                                                        end: 23,
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
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 24,
                                                                    end: 25,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 25,
                                                                end: 26,
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
                                                                            src (ptr): 0x00007fe0fc097840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 27,
                                                                            end: 28,
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
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 23,
                                            end: 29,
                                            as_str(): "(x: T)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 30,
                                                    end: 32,
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
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 33,
                                                                end: 34,
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
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 39,
                                                                end: 40,
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
                                        src (ptr): 0x00007fe0fc097840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                        ),
                                        start: 35,
                                        end: 42,
                                        as_str(): "{\n  x\n}",
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
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 44,
                                            end: 46,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 47,
                                            end: 59,
                                            as_str(): "two_generics",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc097840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                        ),
                                                        start: 59,
                                                        end: 60,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 60,
                                                                    end: 61,
                                                                    as_str(): "A",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            CommaToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 61,
                                                                    end: 62,
                                                                    as_str(): ",",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 63,
                                                                end: 64,
                                                                as_str(): "B",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc097840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                        ),
                                                        start: 64,
                                                        end: 65,
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
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 66,
                                                                        end: 67,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 67,
                                                                    end: 68,
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
                                                                                src (ptr): 0x00007fe0fc097840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 69,
                                                                                end: 70,
                                                                                as_str(): "A",
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
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 70,
                                                                end: 71,
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
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 72,
                                                                    end: 73,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 73,
                                                                end: 74,
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
                                                                            src (ptr): 0x00007fe0fc097840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 75,
                                                                            end: 76,
                                                                            as_str(): "B",
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
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 65,
                                            end: 77,
                                            as_str(): "(a: A, b: B)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 78,
                                                    end: 80,
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
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 81,
                                                                end: 82,
                                                                as_str(): "B",
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
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 87,
                                                                end: 88,
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
                                        src (ptr): 0x00007fe0fc097840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                        ),
                                        start: 83,
                                        end: 90,
                                        as_str(): "{\n  b\n}",
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
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 92,
                                            end: 94,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 95,
                                            end: 109,
                                            as_str(): "three_generics",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc097840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                        ),
                                                        start: 109,
                                                        end: 110,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 110,
                                                                    end: 111,
                                                                    as_str(): "A",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            CommaToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 111,
                                                                    end: 112,
                                                                    as_str(): ",",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 113,
                                                                    end: 114,
                                                                    as_str(): "B",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            CommaToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 114,
                                                                    end: 115,
                                                                    as_str(): ",",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 116,
                                                                end: 117,
                                                                as_str(): "C",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc097840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                        ),
                                                        start: 117,
                                                        end: 118,
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
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 119,
                                                                        end: 120,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 120,
                                                                    end: 121,
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
                                                                                src (ptr): 0x00007fe0fc097840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 122,
                                                                                end: 123,
                                                                                as_str(): "A",
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
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 123,
                                                                end: 124,
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
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 125,
                                                                        end: 126,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 126,
                                                                    end: 127,
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
                                                                                src (ptr): 0x00007fe0fc097840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 128,
                                                                                end: 129,
                                                                                as_str(): "B",
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
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 129,
                                                                end: 130,
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
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 131,
                                                                    end: 132,
                                                                    as_str(): "c",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 132,
                                                                end: 133,
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
                                                                            src (ptr): 0x00007fe0fc097840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 134,
                                                                            end: 135,
                                                                            as_str(): "C",
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
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 118,
                                            end: 136,
                                            as_str(): "(a: A, b: B, c: C)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 137,
                                                    end: 139,
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
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 140,
                                                                end: 141,
                                                                as_str(): "B",
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
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 146,
                                                            end: 149,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 150,
                                                                end: 151,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 151,
                                                                    end: 152,
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
                                                                                src (ptr): 0x00007fe0fc097840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 153,
                                                                                end: 154,
                                                                                as_str(): "A",
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
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 155,
                                                            end: 156,
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
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 157,
                                                                        end: 158,
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
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 158,
                                                            end: 159,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 162,
                                                                end: 163,
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
                                        src (ptr): 0x00007fe0fc097840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                        ),
                                        start: 142,
                                        end: 165,
                                        as_str(): "{\n  let a: A = a;\n  b\n}",
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
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 167,
                                            end: 169,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 170,
                                            end: 174,
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
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 174,
                                            end: 176,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 177,
                                                    end: 179,
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
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 180,
                                                                end: 184,
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
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 189,
                                                            end: 192,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 193,
                                                                end: 194,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 194,
                                                                    end: 195,
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
                                                                                src (ptr): 0x00007fe0fc097840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 203,
                                                            end: 204,
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
                                                                            src (ptr): 0x00007fe0fc097840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 205,
                                                                            end: 213,
                                                                            as_str(): "identity",
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
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc097840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 214,
                                                                                    end: 218,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 213,
                                                                end: 219,
                                                                as_str(): "(true)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 219,
                                                            end: 220,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 223,
                                                            end: 226,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 227,
                                                                end: 228,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 228,
                                                                    end: 229,
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
                                                                                src (ptr): 0x00007fe0fc097840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 230,
                                                                                end: 233,
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
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 237,
                                                            end: 238,
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
                                                                            src (ptr): 0x00007fe0fc097840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 239,
                                                                            end: 247,
                                                                            as_str(): "identity",
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
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc097840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 248,
                                                                                    end: 250,
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
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 247,
                                                                end: 251,
                                                                as_str(): "(10)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 251,
                                                            end: 252,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 255,
                                                            end: 258,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 259,
                                                                end: 260,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 260,
                                                                    end: 261,
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
                                                                                src (ptr): 0x00007fe0fc097840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 262,
                                                                                end: 265,
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 269,
                                                            end: 270,
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
                                                                            src (ptr): 0x00007fe0fc097840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 271,
                                                                            end: 279,
                                                                            as_str(): "identity",
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
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc097840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 280,
                                                                                    end: 282,
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
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 279,
                                                                end: 283,
                                                                as_str(): "(42)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 283,
                                                            end: 284,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 287,
                                                            end: 290,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 291,
                                                                end: 292,
                                                                as_str(): "e",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 292,
                                                                    end: 293,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Str {
                                                                str_token: StrToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 294,
                                                                        end: 297,
                                                                        as_str(): "str",
                                                                    },
                                                                },
                                                                length: SquareBrackets {
                                                                    inner: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc097840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 298,
                                                                                    end: 299,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 297,
                                                                        end: 300,
                                                                        as_str(): "[3]",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 301,
                                                            end: 302,
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
                                                                            src (ptr): 0x00007fe0fc097840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 303,
                                                                            end: 311,
                                                                            as_str(): "identity",
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
                                                                                    src (ptr): 0x00007fe0fc097840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 312,
                                                                                    end: 317,
                                                                                    as_str(): "\"foo\"",
                                                                                },
                                                                                parsed: "foo",
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 311,
                                                                end: 318,
                                                                as_str(): "(\"foo\")",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 318,
                                                            end: 319,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 323,
                                                            end: 326,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 327,
                                                                end: 328,
                                                                as_str(): "f",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 328,
                                                                    end: 329,
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
                                                                                src (ptr): 0x00007fe0fc097840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 330,
                                                                                end: 333,
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 334,
                                                            end: 335,
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
                                                                            src (ptr): 0x00007fe0fc097840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 336,
                                                                            end: 348,
                                                                            as_str(): "two_generics",
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
                                                                                        src (ptr): 0x00007fe0fc097840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 349,
                                                                                        end: 353,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                    kind: True,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc097840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 353,
                                                                                end: 354,
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
                                                                                    src (ptr): 0x00007fe0fc097840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 355,
                                                                                    end: 357,
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
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 348,
                                                                end: 358,
                                                                as_str(): "(true, 10)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 358,
                                                            end: 359,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 362,
                                                            end: 365,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 366,
                                                                end: 367,
                                                                as_str(): "g",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 367,
                                                                    end: 368,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Str {
                                                                str_token: StrToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 369,
                                                                        end: 372,
                                                                        as_str(): "str",
                                                                    },
                                                                },
                                                                length: SquareBrackets {
                                                                    inner: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc097840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 373,
                                                                                    end: 374,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 372,
                                                                        end: 375,
                                                                        as_str(): "[3]",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 376,
                                                            end: 377,
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
                                                                            src (ptr): 0x00007fe0fc097840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 378,
                                                                            end: 392,
                                                                            as_str(): "three_generics",
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
                                                                                        src (ptr): 0x00007fe0fc097840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 393,
                                                                                        end: 397,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                    kind: True,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc097840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 397,
                                                                                end: 398,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        Literal(
                                                                            String(
                                                                                LitString {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc097840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 399,
                                                                                        end: 404,
                                                                                        as_str(): "\"foo\"",
                                                                                    },
                                                                                    parsed: "foo",
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc097840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 404,
                                                                                end: 405,
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
                                                                                    src (ptr): 0x00007fe0fc097840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 406,
                                                                                    end: 408,
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
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 392,
                                                                end: 409,
                                                                as_str(): "(true, \"foo\", 10)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 409,
                                                            end: 410,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 414,
                                                                end: 415,
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
                                        src (ptr): 0x00007fe0fc097840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                        ),
                                        start: 185,
                                        end: 418,
                                        as_str(): "{\n  let a: bool   = identity(true);\n  let b: u32    = identity(10);\n  let c: u64    = identity(42);\n  let e: str[3] = identity(\"foo\");\n\n  let f: u64 = two_generics(true, 10);\n  let g: str[3] = three_generics(true, \"foo\", 10);\n\n  a\n\n}",
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
