Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb14ca11210,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb14ca11210,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
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
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 26,
                                            as_str(): "get_array_pair",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 26,
                                                        end: 27,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 27,
                                                                end: 28,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 28,
                                                        end: 29,
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
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 30,
                                                                        end: 31,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 31,
                                                                    end: 32,
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
                                                                                src (ptr): 0x00007fb14ca11210,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
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
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 34,
                                                                end: 35,
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
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 36,
                                                                    end: 37,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
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
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 39,
                                                                            end: 40,
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
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 29,
                                            end: 41,
                                            as_str(): "(a: T, b: T)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 42,
                                                    end: 44,
                                                    as_str(): "->",
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
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 46,
                                                                            end: 47,
                                                                            as_str(): "T",
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
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 47,
                                                                end: 48,
                                                                as_str(): ";",
                                                            },
                                                        },
                                                        length: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 49,
                                                                        end: 50,
                                                                        as_str(): "2",
                                                                    },
                                                                    parsed: 2,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 45,
                                                        end: 51,
                                                        as_str(): "[T; 2]",
                                                    },
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
                                            Array(
                                                SquareBrackets {
                                                    inner: Sequence(
                                                        Punctuated {
                                                            value_separator_pairs: [
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 59,
                                                                                        end: 60,
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
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 60,
                                                                            end: 61,
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
                                                                                    src (ptr): 0x00007fb14ca11210,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                    ),
                                                                                    start: 62,
                                                                                    end: 63,
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
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 58,
                                                        end: 64,
                                                        as_str(): "[a, b]",
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 52,
                                        end: 66,
                                        as_str(): "{\n    [a, b]\n}",
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
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 68,
                                            end: 70,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 71,
                                            end: 85,
                                            as_str(): "idx_array_pair",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 85,
                                                        end: 86,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 86,
                                                                end: 87,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 87,
                                                        end: 88,
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
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 89,
                                                                        end: 92,
                                                                        as_str(): "ary",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 92,
                                                                    end: 93,
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
                                                                                            src (ptr): 0x00007fb14ca11210,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                            ),
                                                                                            start: 95,
                                                                                            end: 96,
                                                                                            as_str(): "T",
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
                                                                                src (ptr): 0x00007fb14ca11210,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                ),
                                                                                start: 96,
                                                                                end: 97,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 98,
                                                                                        end: 99,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 94,
                                                                        end: 100,
                                                                        as_str(): "[T; 2]",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 100,
                                                                end: 101,
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
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 102,
                                                                    end: 105,
                                                                    as_str(): "idx",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 106,
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
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 107,
                                                                            end: 110,
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
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 88,
                                            end: 111,
                                            as_str(): "(ary: [T; 2], idx: u64)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 112,
                                                    end: 114,
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
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 115,
                                                                end: 116,
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
                                            Index {
                                                target: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 123,
                                                                    end: 126,
                                                                    as_str(): "ary",
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
                                                    inner: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 127,
                                                                        end: 130,
                                                                        as_str(): "idx",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 126,
                                                        end: 131,
                                                        as_str(): "[idx]",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 117,
                                        end: 133,
                                        as_str(): "{\n    ary[idx]\n}",
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
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 135,
                                        end: 141,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 142,
                                        end: 143,
                                        as_str(): "S",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 143,
                                                    end: 144,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 144,
                                                            end: 145,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 145,
                                                    end: 146,
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
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 153,
                                                                end: 154,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 154,
                                                                end: 155,
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
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 157,
                                                                                        end: 158,
                                                                                        as_str(): "T",
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
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 158,
                                                                            end: 159,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                    length: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ca11210,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                    ),
                                                                                    start: 160,
                                                                                    end: 162,
                                                                                    as_str(): "10",
                                                                                },
                                                                                parsed: 10,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 156,
                                                                    end: 163,
                                                                    as_str(): "[T; 10]",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 163,
                                                        end: 164,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 147,
                                        end: 166,
                                        as_str(): "{\n    a: [T; 10],\n}",
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
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 168,
                                            end: 170,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 171,
                                            end: 175,
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
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 175,
                                            end: 177,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 178,
                                                    end: 180,
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
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 181,
                                                                end: 185,
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
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 192,
                                                            end: 195,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 196,
                                                                end: 203,
                                                                as_str(): "ary_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 203,
                                                                    end: 204,
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
                                                                                            src (ptr): 0x00007fb14ca11210,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                            ),
                                                                                            start: 206,
                                                                                            end: 209,
                                                                                            as_str(): "u64",
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
                                                                                src (ptr): 0x00007fb14ca11210,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                ),
                                                                                start: 209,
                                                                                end: 210,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 211,
                                                                                        end: 212,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 205,
                                                                        end: 213,
                                                                        as_str(): "[u64; 2]",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 214,
                                                            end: 215,
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
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 216,
                                                                            end: 230,
                                                                            as_str(): "get_array_pair",
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
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 231,
                                                                                        end: 232,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ca11210,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                ),
                                                                                start: 232,
                                                                                end: 233,
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
                                                                                    src (ptr): 0x00007fb14ca11210,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                    ),
                                                                                    start: 234,
                                                                                    end: 235,
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
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 230,
                                                                end: 236,
                                                                as_str(): "(1, 2)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 236,
                                                            end: 237,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 243,
                                                            end: 246,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 247,
                                                                end: 248,
                                                                as_str(): "s",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 249,
                                                            end: 250,
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
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 251,
                                                                        end: 252,
                                                                        as_str(): "S",
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
                                                                                src (ptr): 0x00007fb14ca11210,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                ),
                                                                                start: 263,
                                                                                end: 264,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 264,
                                                                                        end: 265,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Array(
                                                                                    SquareBrackets {
                                                                                        inner: Repeat {
                                                                                            value: Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14ca11210,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 267,
                                                                                                            end: 269,
                                                                                                            as_str(): "0_",
                                                                                                        },
                                                                                                        parsed: 0,
                                                                                                        ty_opt: Some(
                                                                                                            (
                                                                                                                U64,
                                                                                                                Span {
                                                                                                                    src (ptr): 0x00007fb14ca11210,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 269,
                                                                                                                    end: 272,
                                                                                                                    as_str(): "u64",
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                            semicolon_token: SemicolonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14ca11210,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 272,
                                                                                                    end: 273,
                                                                                                    as_str(): ";",
                                                                                                },
                                                                                            },
                                                                                            length: Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14ca11210,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 274,
                                                                                                            end: 276,
                                                                                                            as_str(): "10",
                                                                                                        },
                                                                                                        parsed: 10,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ca11210,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                            ),
                                                                                            start: 266,
                                                                                            end: 277,
                                                                                            as_str(): "[0_u64; 10]",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 253,
                                                                end: 283,
                                                                as_str(): "{\n        a: [0_u64; 10]\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
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
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 289,
                                                            end: 292,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 293,
                                                                end: 294,
                                                                as_str(): "t",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 295,
                                                            end: 296,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Index {
                                                        target: Parens(
                                                            Parens {
                                                                inner: FieldProjection {
                                                                    target: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 298,
                                                                                        end: 299,
                                                                                        as_str(): "s",
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
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 299,
                                                                            end: 300,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 300,
                                                                            end: 301,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 297,
                                                                    end: 302,
                                                                    as_str(): "(s.a)",
                                                                },
                                                            },
                                                        ),
                                                        arg: SquareBrackets {
                                                            inner: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 303,
                                                                            end: 304,
                                                                            as_str(): "9",
                                                                        },
                                                                        parsed: 9,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 302,
                                                                end: 305,
                                                                as_str(): "[9]",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 305,
                                                            end: 306,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 312,
                                                            end: 315,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 316,
                                                                end: 324,
                                                                as_str(): "ary_bool",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 325,
                                                            end: 326,
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
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 327,
                                                                            end: 341,
                                                                            as_str(): "get_array_pair",
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
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 342,
                                                                                        end: 347,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                    kind: False,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ca11210,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                ),
                                                                                start: 347,
                                                                                end: 348,
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
                                                                                    src (ptr): 0x00007fb14ca11210,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                    ),
                                                                                    start: 349,
                                                                                    end: 353,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 341,
                                                                end: 354,
                                                                as_str(): "(false, true)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 354,
                                                            end: 355,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 360,
                                                                    end: 374,
                                                                    as_str(): "idx_array_pair",
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
                                                                                    src (ptr): 0x00007fb14ca11210,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                    ),
                                                                                    start: 375,
                                                                                    end: 383,
                                                                                    as_str(): "ary_bool",
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
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 383,
                                                                        end: 384,
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
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 385,
                                                                            end: 386,
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
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 374,
                                                        end: 387,
                                                        as_str(): "(ary_bool, 1)",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 186,
                                        end: 389,
                                        as_str(): "{\n    let ary_u64: [u64; 2] = get_array_pair(1, 2);\n\n    let s = S {\n        a: [0_u64; 10]\n    };\n    let t = (s.a)[9];\n\n    let ary_bool = get_array_pair(false, true);\n    idx_array_pair(ary_bool, 1)\n}",
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
