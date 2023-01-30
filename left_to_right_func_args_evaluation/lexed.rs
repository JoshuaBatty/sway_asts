Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0bd11adc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0bd11adc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
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
                                            src (ptr): 0x00007fe0bd11adc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                            ),
                                            start: 90,
                                            end: 92,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0bd11adc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                            ),
                                            start: 93,
                                            end: 97,
                                            as_str(): "func",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
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
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 98,
                                                                        end: 99,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 99,
                                                                    end: 100,
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
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 101,
                                                                                end: 104,
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
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 104,
                                                                end: 105,
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
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 106,
                                                                        end: 107,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 107,
                                                                    end: 108,
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
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 112,
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
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 112,
                                                                end: 113,
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
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 114,
                                                                        end: 115,
                                                                        as_str(): "c",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
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
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
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
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 120,
                                                                end: 121,
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
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 122,
                                                                    end: 123,
                                                                    as_str(): "d",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 123,
                                                                end: 124,
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
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 125,
                                                                            end: 128,
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
                                            src (ptr): 0x00007fe0bd11adc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                            ),
                                            start: 97,
                                            end: 129,
                                            as_str(): "(a: u64, b: u64, c: u64, d: u64)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 130,
                                                    end: 132,
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
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 133,
                                                                end: 136,
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 143,
                                                                end: 144,
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
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0bd11adc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                        ),
                                        start: 137,
                                        end: 146,
                                        as_str(): "{\n    d\n}",
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
                                        src (ptr): 0x00007fe0bd11adc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                        ),
                                        start: 148,
                                        end: 154,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0bd11adc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                        ),
                                        start: 155,
                                        end: 163,
                                        as_str(): "MyStruct",
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
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 170,
                                                                end: 171,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
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
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 173,
                                                                            end: 176,
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
                                                        src (ptr): 0x00007fe0bd11adc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                        ),
                                                        start: 176,
                                                        end: 177,
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
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 182,
                                                                end: 183,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 183,
                                                                end: 184,
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
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 185,
                                                                            end: 188,
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
                                                        src (ptr): 0x00007fe0bd11adc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                        ),
                                                        start: 188,
                                                        end: 189,
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
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 194,
                                                                end: 195,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 195,
                                                                end: 196,
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
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 197,
                                                                            end: 200,
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
                                                        src (ptr): 0x00007fe0bd11adc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                        ),
                                                        start: 200,
                                                        end: 201,
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
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 206,
                                                                end: 207,
                                                                as_str(): "d",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 207,
                                                                end: 208,
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
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 209,
                                                                            end: 212,
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
                                                        src (ptr): 0x00007fe0bd11adc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                        ),
                                                        start: 212,
                                                        end: 213,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0bd11adc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                        ),
                                        start: 164,
                                        end: 215,
                                        as_str(): "{\n    a: u64,\n    b: u64,\n    c: u64,\n    d: u64,\n}",
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
                                            src (ptr): 0x00007fe0bd11adc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                            ),
                                            start: 217,
                                            end: 219,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0bd11adc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                            ),
                                            start: 220,
                                            end: 224,
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
                                            src (ptr): 0x00007fe0bd11adc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                            ),
                                            start: 224,
                                            end: 226,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 227,
                                                    end: 229,
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
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 230,
                                                                end: 234,
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
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 241,
                                                            end: 244,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 245,
                                                                    end: 248,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 249,
                                                                end: 250,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 250,
                                                                    end: 251,
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
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 252,
                                                                                end: 255,
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
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 256,
                                                            end: 257,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 258,
                                                                    end: 259,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 259,
                                                            end: 260,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 303,
                                                            end: 306,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 307,
                                                                end: 315,
                                                                as_str(): "func_res",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 316,
                                                            end: 317,
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
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 326,
                                                                            end: 330,
                                                                            as_str(): "func",
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
                                                                        Block(
                                                                            Braces {
                                                                                inner: CodeBlockContents {
                                                                                    statements: [
                                                                                        Expr {
                                                                                            expr: Reassignment {
                                                                                                assignable: Var(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 362,
                                                                                                            end: 363,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                reassignment_op: ReassignmentOp {
                                                                                                    variant: Equals,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
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
                                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                ),
                                                                                                                start: 366,
                                                                                                                end: 367,
                                                                                                                as_str(): "1",
                                                                                                            },
                                                                                                            parsed: 1,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                            semicolon_token_opt: Some(
                                                                                                SemicolonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 367,
                                                                                                        end: 368,
                                                                                                        as_str(): ";",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                    ],
                                                                                    final_expr_opt: Some(
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 385,
                                                                                                        end: 386,
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
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 344,
                                                                                    end: 400,
                                                                                    as_str(): "{\n                x = 1;\n                0\n            }",
                                                                                },
                                                                            },
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 400,
                                                                                end: 401,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        Block(
                                                                            Braces {
                                                                                inner: CodeBlockContents {
                                                                                    statements: [
                                                                                        Expr {
                                                                                            expr: Reassignment {
                                                                                                assignable: Var(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 432,
                                                                                                            end: 433,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                reassignment_op: ReassignmentOp {
                                                                                                    variant: Equals,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 434,
                                                                                                        end: 435,
                                                                                                        as_str(): "=",
                                                                                                    },
                                                                                                },
                                                                                                expr: Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                ),
                                                                                                                start: 436,
                                                                                                                end: 437,
                                                                                                                as_str(): "2",
                                                                                                            },
                                                                                                            parsed: 2,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                            semicolon_token_opt: Some(
                                                                                                SemicolonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 437,
                                                                                                        end: 438,
                                                                                                        as_str(): ";",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                    ],
                                                                                    final_expr_opt: Some(
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 455,
                                                                                                        end: 456,
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
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 414,
                                                                                    end: 470,
                                                                                    as_str(): "{\n                x = 2;\n                0\n            }",
                                                                                },
                                                                            },
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 470,
                                                                                end: 471,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        Block(
                                                                            Braces {
                                                                                inner: CodeBlockContents {
                                                                                    statements: [
                                                                                        Expr {
                                                                                            expr: Reassignment {
                                                                                                assignable: Var(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 502,
                                                                                                            end: 503,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                reassignment_op: ReassignmentOp {
                                                                                                    variant: Equals,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 504,
                                                                                                        end: 505,
                                                                                                        as_str(): "=",
                                                                                                    },
                                                                                                },
                                                                                                expr: Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                ),
                                                                                                                start: 506,
                                                                                                                end: 507,
                                                                                                                as_str(): "3",
                                                                                                            },
                                                                                                            parsed: 3,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                            semicolon_token_opt: Some(
                                                                                                SemicolonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 507,
                                                                                                        end: 508,
                                                                                                        as_str(): ";",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                    ],
                                                                                    final_expr_opt: Some(
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 525,
                                                                                                        end: 526,
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
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 484,
                                                                                    end: 540,
                                                                                    as_str(): "{\n                x = 3;\n                0\n            }",
                                                                                },
                                                                            },
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 540,
                                                                                end: 541,
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
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 554,
                                                                                        end: 555,
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
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 330,
                                                                end: 565,
                                                                as_str(): "(\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                0\n            },\n            {\n                x = 3;\n                0\n            },\n            x\n        )",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 565,
                                                            end: 566,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: Var(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 596,
                                                                end: 597,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 598,
                                                            end: 599,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 600,
                                                                    end: 601,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 601,
                                                            end: 602,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 607,
                                                            end: 610,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 611,
                                                                end: 620,
                                                                as_str(): "tuple_res",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 621,
                                                            end: 622,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Tuple(
                                                        Parens {
                                                            inner: Cons {
                                                                head: Block(
                                                                    Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [
                                                                                Expr {
                                                                                    expr: Reassignment {
                                                                                        assignable: Var(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                    ),
                                                                                                    start: 663,
                                                                                                    end: 664,
                                                                                                    as_str(): "x",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        reassignment_op: ReassignmentOp {
                                                                                            variant: Equals,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 665,
                                                                                                end: 666,
                                                                                                as_str(): "=",
                                                                                            },
                                                                                        },
                                                                                        expr: Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 667,
                                                                                                        end: 668,
                                                                                                        as_str(): "1",
                                                                                                    },
                                                                                                    parsed: 1,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    semicolon_token_opt: Some(
                                                                                        SemicolonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 668,
                                                                                                end: 669,
                                                                                                as_str(): ";",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                            ],
                                                                            final_expr_opt: Some(
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 686,
                                                                                                end: 687,
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
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 645,
                                                                            end: 701,
                                                                            as_str(): "{\n                x = 1;\n                0\n            }",
                                                                        },
                                                                    },
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 701,
                                                                        end: 702,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                                tail: Punctuated {
                                                                    value_separator_pairs: [
                                                                        (
                                                                            Block(
                                                                                Braces {
                                                                                    inner: CodeBlockContents {
                                                                                        statements: [
                                                                                            Expr {
                                                                                                expr: Reassignment {
                                                                                                    assignable: Var(
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                ),
                                                                                                                start: 733,
                                                                                                                end: 734,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    reassignment_op: ReassignmentOp {
                                                                                                        variant: Equals,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 735,
                                                                                                            end: 736,
                                                                                                            as_str(): "=",
                                                                                                        },
                                                                                                    },
                                                                                                    expr: Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 737,
                                                                                                                    end: 738,
                                                                                                                    as_str(): "2",
                                                                                                                },
                                                                                                                parsed: 2,
                                                                                                                ty_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                                semicolon_token_opt: Some(
                                                                                                    SemicolonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 738,
                                                                                                            end: 739,
                                                                                                            as_str(): ";",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                        ],
                                                                                        final_expr_opt: Some(
                                                                                            Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 756,
                                                                                                            end: 757,
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
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 715,
                                                                                        end: 771,
                                                                                        as_str(): "{\n                x = 2;\n                1\n            }",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 771,
                                                                                    end: 772,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            Block(
                                                                                Braces {
                                                                                    inner: CodeBlockContents {
                                                                                        statements: [
                                                                                            Expr {
                                                                                                expr: Reassignment {
                                                                                                    assignable: Var(
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                ),
                                                                                                                start: 803,
                                                                                                                end: 804,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    reassignment_op: ReassignmentOp {
                                                                                                        variant: Equals,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 805,
                                                                                                            end: 806,
                                                                                                            as_str(): "=",
                                                                                                        },
                                                                                                    },
                                                                                                    expr: Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 807,
                                                                                                                    end: 808,
                                                                                                                    as_str(): "3",
                                                                                                                },
                                                                                                                parsed: 3,
                                                                                                                ty_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                                semicolon_token_opt: Some(
                                                                                                    SemicolonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 808,
                                                                                                            end: 809,
                                                                                                            as_str(): ";",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                        ],
                                                                                        final_expr_opt: Some(
                                                                                            Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 826,
                                                                                                            end: 827,
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
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 785,
                                                                                        end: 841,
                                                                                        as_str(): "{\n                x = 3;\n                2\n            }",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 841,
                                                                                    end: 842,
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
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 855,
                                                                                            end: 856,
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
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 631,
                                                                end: 866,
                                                                as_str(): "(\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                1\n            },\n            {\n                x = 3;\n                2\n            },\n            x\n        )",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 866,
                                                            end: 867,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: Var(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 898,
                                                                end: 899,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 900,
                                                            end: 901,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 902,
                                                                    end: 903,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 903,
                                                            end: 904,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 909,
                                                            end: 912,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 913,
                                                                end: 923,
                                                                as_str(): "struct_res",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 924,
                                                            end: 925,
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
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 934,
                                                                        end: 942,
                                                                        as_str(): "MyStruct",
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
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 957,
                                                                                    end: 958,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 958,
                                                                                            end: 959,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Block(
                                                                                        Braces {
                                                                                            inner: CodeBlockContents {
                                                                                                statements: [
                                                                                                    Expr {
                                                                                                        expr: Reassignment {
                                                                                                            assignable: Var(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 978,
                                                                                                                        end: 979,
                                                                                                                        as_str(): "x",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            reassignment_op: ReassignmentOp {
                                                                                                                variant: Equals,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 980,
                                                                                                                    end: 981,
                                                                                                                    as_str(): "=",
                                                                                                                },
                                                                                                            },
                                                                                                            expr: Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 982,
                                                                                                                            end: 983,
                                                                                                                            as_str(): "1",
                                                                                                                        },
                                                                                                                        parsed: 1,
                                                                                                                        ty_opt: None,
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        semicolon_token_opt: Some(
                                                                                                            SemicolonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 983,
                                                                                                                    end: 984,
                                                                                                                    as_str(): ";",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                ],
                                                                                                final_expr_opt: Some(
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1001,
                                                                                                                    end: 1002,
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
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 960,
                                                                                                end: 1016,
                                                                                                as_str(): "{\n                x = 1;\n                0\n            }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 1016,
                                                                                end: 1017,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 1030,
                                                                                    end: 1031,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 1031,
                                                                                            end: 1032,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Block(
                                                                                        Braces {
                                                                                            inner: CodeBlockContents {
                                                                                                statements: [
                                                                                                    Expr {
                                                                                                        expr: Reassignment {
                                                                                                            assignable: Var(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1051,
                                                                                                                        end: 1052,
                                                                                                                        as_str(): "x",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            reassignment_op: ReassignmentOp {
                                                                                                                variant: Equals,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1053,
                                                                                                                    end: 1054,
                                                                                                                    as_str(): "=",
                                                                                                                },
                                                                                                            },
                                                                                                            expr: Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1055,
                                                                                                                            end: 1056,
                                                                                                                            as_str(): "2",
                                                                                                                        },
                                                                                                                        parsed: 2,
                                                                                                                        ty_opt: None,
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        semicolon_token_opt: Some(
                                                                                                            SemicolonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1056,
                                                                                                                    end: 1057,
                                                                                                                    as_str(): ";",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                ],
                                                                                                final_expr_opt: Some(
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1074,
                                                                                                                    end: 1075,
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
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 1033,
                                                                                                end: 1089,
                                                                                                as_str(): "{\n                x = 2;\n                1\n            }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 1089,
                                                                                end: 1090,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 1103,
                                                                                    end: 1104,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 1104,
                                                                                            end: 1105,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Block(
                                                                                        Braces {
                                                                                            inner: CodeBlockContents {
                                                                                                statements: [
                                                                                                    Expr {
                                                                                                        expr: Reassignment {
                                                                                                            assignable: Var(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1124,
                                                                                                                        end: 1125,
                                                                                                                        as_str(): "x",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            reassignment_op: ReassignmentOp {
                                                                                                                variant: Equals,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1126,
                                                                                                                    end: 1127,
                                                                                                                    as_str(): "=",
                                                                                                                },
                                                                                                            },
                                                                                                            expr: Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1128,
                                                                                                                            end: 1129,
                                                                                                                            as_str(): "3",
                                                                                                                        },
                                                                                                                        parsed: 3,
                                                                                                                        ty_opt: None,
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        semicolon_token_opt: Some(
                                                                                                            SemicolonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1129,
                                                                                                                    end: 1130,
                                                                                                                    as_str(): ";",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                ],
                                                                                                final_expr_opt: Some(
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1147,
                                                                                                                    end: 1148,
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
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 1106,
                                                                                                end: 1162,
                                                                                                as_str(): "{\n                x = 3;\n                2\n            }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 1162,
                                                                                end: 1163,
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
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 1176,
                                                                                end: 1177,
                                                                                as_str(): "d",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 1177,
                                                                                        end: 1178,
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
                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1179,
                                                                                                    end: 1180,
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
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 943,
                                                                end: 1190,
                                                                as_str(): "{\n            a: {\n                x = 1;\n                0\n            },\n            b: {\n                x = 2;\n                1\n            },\n            c: {\n                x = 3;\n                2\n            },\n            d: x\n        }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 1190,
                                                            end: 1191,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 1197,
                                                            end: 1203,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        LogicalAnd {
                                                            lhs: LogicalAnd {
                                                                lhs: Parens(
                                                                    Parens {
                                                                        inner: Equal {
                                                                            lhs: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 1205,
                                                                                                end: 1213,
                                                                                                as_str(): "func_res",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [],
                                                                                    incomplete_suffix: false,
                                                                                },
                                                                            ),
                                                                            double_eq_token: DoubleEqToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 1214,
                                                                                    end: 1216,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            rhs: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 1217,
                                                                                            end: 1218,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                        parsed: 3,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 1204,
                                                                            end: 1219,
                                                                            as_str(): "(func_res == 3)",
                                                                        },
                                                                    },
                                                                ),
                                                                double_ampersand_token: DoubleAmpersandToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 1220,
                                                                        end: 1222,
                                                                        as_str(): "&&",
                                                                    },
                                                                },
                                                                rhs: Parens(
                                                                    Parens {
                                                                        inner: Equal {
                                                                            lhs: TupleFieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1224,
                                                                                                    end: 1233,
                                                                                                    as_str(): "tuple_res",
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
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 1233,
                                                                                        end: 1234,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                field: 3,
                                                                                field_span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 1234,
                                                                                    end: 1235,
                                                                                    as_str(): "3",
                                                                                },
                                                                            },
                                                                            double_eq_token: DoubleEqToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 1236,
                                                                                    end: 1238,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            rhs: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 1239,
                                                                                            end: 1240,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                        parsed: 3,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 1223,
                                                                            end: 1241,
                                                                            as_str(): "(tuple_res.3 == 3)",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            double_ampersand_token: DoubleAmpersandToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 1242,
                                                                    end: 1244,
                                                                    as_str(): "&&",
                                                                },
                                                            },
                                                            rhs: Parens(
                                                                Parens {
                                                                    inner: Equal {
                                                                        lhs: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 1246,
                                                                                                end: 1256,
                                                                                                as_str(): "struct_res",
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
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 1256,
                                                                                    end: 1257,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 1257,
                                                                                    end: 1258,
                                                                                    as_str(): "d",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 1259,
                                                                                end: 1261,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 1262,
                                                                                        end: 1263,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                    parsed: 3,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 1245,
                                                                        end: 1264,
                                                                        as_str(): "(struct_res.d == 3)",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 1264,
                                                            end: 1265,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0bd11adc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                        ),
                                        start: 235,
                                        end: 1267,
                                        as_str(): "{\n    let mut x: u64 = 0;\n\n    // function arguments evaluation\n    let func_res =\n        func(\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                0\n            },\n            {\n                x = 3;\n                0\n            },\n            x\n        );\n\n    // tuple evaluation\n    x = 0;\n    let tuple_res =\n        (\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                1\n            },\n            {\n                x = 3;\n                2\n            },\n            x\n        );\n\n    // struct evaluation\n    x = 0;\n    let struct_res =\n        MyStruct {\n            a: {\n                x = 1;\n                0\n            },\n            b: {\n                x = 2;\n                1\n            },\n            c: {\n                x = 3;\n                2\n            },\n            d: x\n        };\n\n    return (func_res == 3) && (tuple_res.3 == 3) && (struct_res.d == 3);\n}",
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
