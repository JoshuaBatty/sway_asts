Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0acfed220,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 12,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0acfed220,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0acfed220,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0acfed220,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 27,
                                                as_str(): "constants",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0acfed220,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                ),
                                                start: 27,
                                                end: 29,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 29,
                                                    end: 38,
                                                    as_str(): "ZERO_B256",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 38,
                                        end: 39,
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
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 41,
                                        end: 47,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 48,
                                        end: 49,
                                        as_str(): "Z",
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
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 56,
                                                                end: 57,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 57,
                                                                end: 58,
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
                                                                            src (ptr): 0x00007fe0acfed220,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                            ),
                                                                            start: 59,
                                                                            end: 62,
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
                                                        src (ptr): 0x00007fe0acfed220,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                        ),
                                                        start: 62,
                                                        end: 63,
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
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 68,
                                                                end: 69,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
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
                                                                            src (ptr): 0x00007fe0acfed220,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                            ),
                                                                            start: 71,
                                                                            end: 74,
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
                                                        src (ptr): 0x00007fe0acfed220,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                        ),
                                                        start: 74,
                                                        end: 75,
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
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 80,
                                                                end: 81,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 81,
                                                                end: 82,
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
                                                                            src (ptr): 0x00007fe0acfed220,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
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
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0acfed220,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                        ),
                                                        start: 86,
                                                        end: 87,
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
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 92,
                                                                end: 93,
                                                                as_str(): "d",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 93,
                                                                end: 94,
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
                                                                            src (ptr): 0x00007fe0acfed220,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                            ),
                                                                            start: 95,
                                                                            end: 98,
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
                                                        src (ptr): 0x00007fe0acfed220,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
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
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 50,
                                        end: 101,
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
                                            src (ptr): 0x00007fe0acfed220,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                            ),
                                            start: 103,
                                            end: 105,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0acfed220,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                            ),
                                            start: 106,
                                            end: 110,
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
                                            src (ptr): 0x00007fe0acfed220,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                            ),
                                            start: 110,
                                            end: 112,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 113,
                                                    end: 115,
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
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 116,
                                                                end: 119,
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 330,
                                                            end: 333,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 334,
                                                                end: 337,
                                                                as_str(): "zzz",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 338,
                                                            end: 339,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 340,
                                                                        end: 341,
                                                                        as_str(): "Z",
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
                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                    ),
                                                                                    start: 352,
                                                                                    end: 353,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                            ),
                                                                                            start: 353,
                                                                                            end: 354,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                    ),
                                                                                                    start: 355,
                                                                                                    end: 356,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                ),
                                                                                start: 356,
                                                                                end: 357,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                    ),
                                                                                    start: 366,
                                                                                    end: 367,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                            ),
                                                                                            start: 367,
                                                                                            end: 368,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                    ),
                                                                                                    start: 369,
                                                                                                    end: 370,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                                parsed: 2,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                ),
                                                                                start: 370,
                                                                                end: 371,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                    ),
                                                                                    start: 380,
                                                                                    end: 381,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                            ),
                                                                                            start: 381,
                                                                                            end: 382,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                    ),
                                                                                                    start: 383,
                                                                                                    end: 384,
                                                                                                    as_str(): "3",
                                                                                                },
                                                                                                parsed: 3,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                ),
                                                                                start: 384,
                                                                                end: 385,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                    ),
                                                                                    start: 394,
                                                                                    end: 395,
                                                                                    as_str(): "d",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                            ),
                                                                                            start: 395,
                                                                                            end: 396,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                    ),
                                                                                                    start: 397,
                                                                                                    end: 398,
                                                                                                    as_str(): "4",
                                                                                                },
                                                                                                parsed: 4,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                ),
                                                                                start: 398,
                                                                                end: 399,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 342,
                                                                end: 405,
                                                                as_str(): "{\n        a: 1,\n        b: 2,\n        c: 3,\n        d: 4,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 405,
                                                            end: 406,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 449,
                                                            end: 452,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 453,
                                                                end: 455,
                                                                as_str(): "z1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 456,
                                                            end: 457,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 458,
                                                                    end: 459,
                                                                    as_str(): "5",
                                                                },
                                                                parsed: 5,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 459,
                                                            end: 460,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 465,
                                                            end: 468,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 469,
                                                                end: 471,
                                                                as_str(): "z2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 472,
                                                            end: 473,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 474,
                                                                    end: 475,
                                                                    as_str(): "6",
                                                                },
                                                                parsed: 6,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 475,
                                                            end: 476,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 538,
                                                                        end: 541,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 541,
                                                            end: 543,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 543,
                                                            end: 544,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 549,
                                                                        end: 552,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 552,
                                                            end: 554,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 554,
                                                            end: 555,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 560,
                                                                        end: 563,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 563,
                                                            end: 565,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 565,
                                                            end: 566,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 571,
                                                                        end: 574,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 574,
                                                            end: 576,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 576,
                                                            end: 577,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 582,
                                                                        end: 585,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 585,
                                                            end: 587,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 587,
                                                            end: 588,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 593,
                                                                        end: 596,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 596,
                                                            end: 598,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 598,
                                                            end: 599,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 604,
                                                                        end: 607,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 607,
                                                            end: 609,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 609,
                                                            end: 610,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 615,
                                                                        end: 618,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 618,
                                                            end: 620,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 620,
                                                            end: 621,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 626,
                                                                        end: 629,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 629,
                                                            end: 631,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 631,
                                                            end: 632,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 637,
                                                                        end: 640,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 640,
                                                            end: 642,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 642,
                                                            end: 643,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 648,
                                                                        end: 651,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 651,
                                                            end: 653,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 653,
                                                            end: 654,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 659,
                                                                        end: 662,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 662,
                                                            end: 664,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 664,
                                                            end: 665,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 670,
                                                                        end: 673,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 673,
                                                            end: 675,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 675,
                                                            end: 676,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 681,
                                                                        end: 684,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 684,
                                                            end: 686,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 686,
                                                            end: 687,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 692,
                                                                        end: 695,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 695,
                                                            end: 697,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 697,
                                                            end: 698,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 703,
                                                                        end: 706,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 706,
                                                            end: 708,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 708,
                                                            end: 709,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 714,
                                                                        end: 717,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 717,
                                                            end: 719,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 719,
                                                            end: 720,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 725,
                                                                        end: 728,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 728,
                                                            end: 730,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 730,
                                                            end: 731,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 736,
                                                                        end: 739,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 739,
                                                            end: 741,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 741,
                                                            end: 742,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 747,
                                                                        end: 750,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 750,
                                                            end: 752,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 752,
                                                            end: 753,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 758,
                                                                        end: 761,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 761,
                                                            end: 763,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 763,
                                                            end: 764,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 769,
                                                                        end: 772,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 772,
                                                            end: 774,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 774,
                                                            end: 775,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 780,
                                                                        end: 783,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 783,
                                                            end: 785,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 785,
                                                            end: 786,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 791,
                                                                        end: 794,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 794,
                                                            end: 796,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 796,
                                                            end: 797,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 802,
                                                                        end: 805,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 805,
                                                            end: 807,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 807,
                                                            end: 808,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 813,
                                                                        end: 816,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 816,
                                                            end: 818,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 818,
                                                            end: 819,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 824,
                                                                        end: 827,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 827,
                                                            end: 829,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 829,
                                                            end: 830,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 835,
                                                                        end: 838,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 838,
                                                            end: 840,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 840,
                                                            end: 841,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 846,
                                                                        end: 849,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 849,
                                                            end: 851,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 851,
                                                            end: 852,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 857,
                                                                        end: 860,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 860,
                                                            end: 862,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 862,
                                                            end: 863,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 868,
                                                                        end: 871,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 871,
                                                            end: 873,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 873,
                                                            end: 874,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 879,
                                                                        end: 882,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 882,
                                                            end: 884,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 884,
                                                            end: 885,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 890,
                                                                        end: 893,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 893,
                                                            end: 895,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 895,
                                                            end: 896,
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
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 901,
                                                                        end: 904,
                                                                        as_str(): "foo",
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 904,
                                                            end: 906,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 906,
                                                            end: 907,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Return {
                                                return_token: ReturnToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0acfed220,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                        ),
                                                        start: 913,
                                                        end: 919,
                                                        as_str(): "return",
                                                    },
                                                },
                                                expr_opt: Some(
                                                    Add {
                                                        lhs: Add {
                                                            lhs: Add {
                                                                lhs: Add {
                                                                    lhs: Add {
                                                                        lhs: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                ),
                                                                                                start: 920,
                                                                                                end: 923,
                                                                                                as_str(): "zzz",
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
                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                    ),
                                                                                    start: 923,
                                                                                    end: 924,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                    ),
                                                                                    start: 924,
                                                                                    end: 925,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        add_token: AddToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                ),
                                                                                start: 926,
                                                                                end: 927,
                                                                                as_str(): "+",
                                                                            },
                                                                        },
                                                                        rhs: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                ),
                                                                                                start: 928,
                                                                                                end: 931,
                                                                                                as_str(): "zzz",
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
                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                    ),
                                                                                    start: 931,
                                                                                    end: 932,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                    ),
                                                                                    start: 932,
                                                                                    end: 933,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                    },
                                                                    add_token: AddToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0acfed220,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                            ),
                                                                            start: 934,
                                                                            end: 935,
                                                                            as_str(): "+",
                                                                        },
                                                                    },
                                                                    rhs: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                            ),
                                                                                            start: 936,
                                                                                            end: 939,
                                                                                            as_str(): "zzz",
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
                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                ),
                                                                                start: 939,
                                                                                end: 940,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                ),
                                                                                start: 940,
                                                                                end: 941,
                                                                                as_str(): "c",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                },
                                                                add_token: AddToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 942,
                                                                        end: 943,
                                                                        as_str(): "+",
                                                                    },
                                                                },
                                                                rhs: FieldProjection {
                                                                    target: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0acfed220,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                        ),
                                                                                        start: 944,
                                                                                        end: 947,
                                                                                        as_str(): "zzz",
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
                                                                            src (ptr): 0x00007fe0acfed220,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                            ),
                                                                            start: 947,
                                                                            end: 948,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0acfed220,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                            ),
                                                                            start: 948,
                                                                            end: 949,
                                                                            as_str(): "d",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                            },
                                                            add_token: AddToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 950,
                                                                    end: 951,
                                                                    as_str(): "+",
                                                                },
                                                            },
                                                            rhs: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                ),
                                                                                start: 952,
                                                                                end: 954,
                                                                                as_str(): "z1",
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
                                                        add_token: AddToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 955,
                                                                end: 956,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                        rhs: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0acfed220,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                            ),
                                                                            start: 957,
                                                                            end: 959,
                                                                            as_str(): "z2",
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
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 120,
                                        end: 971,
                                        as_str(): "{\n    // Chosen names force these variables to show up last in the list of locals in IR so they will\n    // be allocated last and require the highest offset to be computed\n\n    // Test get_ptr large offset\n    let zzz = Z {\n        a: 1,\n        b: 2,\n        c: 3,\n        d: 4,\n    };\n\n    // Test LW/SW with large offsets\n    let z1 = 5;\n    let z2 = 6;\n\n    // Add enough stack variables to reach > 4096 words\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n\n    return zzz.a + zzz.b + zzz.c + zzz.d + z1 + z2 // get 16\n}",
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
                                            src (ptr): 0x00007fe0acfed220,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                            ),
                                            start: 973,
                                            end: 975,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0acfed220,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                            ),
                                            start: 976,
                                            end: 979,
                                            as_str(): "foo",
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
                                            src (ptr): 0x00007fe0acfed220,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                            ),
                                            start: 979,
                                            end: 981,
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
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 988,
                                                            end: 991,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 992,
                                                                end: 993,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 994,
                                                            end: 995,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 996,
                                                                    end: 997,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 997,
                                                            end: 998,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1003,
                                                            end: 1006,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1007,
                                                                end: 1008,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1009,
                                                            end: 1010,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1011,
                                                                    end: 1012,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1012,
                                                            end: 1013,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1018,
                                                            end: 1021,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1022,
                                                                end: 1023,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1024,
                                                            end: 1025,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1026,
                                                                    end: 1027,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1027,
                                                            end: 1028,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1033,
                                                            end: 1036,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1037,
                                                                end: 1038,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1039,
                                                            end: 1040,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1041,
                                                                    end: 1042,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1042,
                                                            end: 1043,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1048,
                                                            end: 1051,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1052,
                                                                end: 1053,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1054,
                                                            end: 1055,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1056,
                                                                    end: 1057,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1057,
                                                            end: 1058,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1063,
                                                            end: 1066,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1067,
                                                                end: 1068,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1069,
                                                            end: 1070,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1071,
                                                                    end: 1072,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1072,
                                                            end: 1073,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1078,
                                                            end: 1081,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1082,
                                                                end: 1083,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1084,
                                                            end: 1085,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1086,
                                                                    end: 1087,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1087,
                                                            end: 1088,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1093,
                                                            end: 1096,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1097,
                                                                end: 1098,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1099,
                                                            end: 1100,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1101,
                                                                    end: 1102,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1102,
                                                            end: 1103,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1108,
                                                            end: 1111,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1112,
                                                                end: 1113,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1114,
                                                            end: 1115,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1116,
                                                                    end: 1117,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1117,
                                                            end: 1118,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1123,
                                                            end: 1126,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1127,
                                                                end: 1128,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1129,
                                                            end: 1130,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1131,
                                                                    end: 1132,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1132,
                                                            end: 1133,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1138,
                                                            end: 1141,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1142,
                                                                end: 1143,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1144,
                                                            end: 1145,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1146,
                                                                    end: 1147,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1147,
                                                            end: 1148,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1153,
                                                            end: 1156,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1157,
                                                                end: 1158,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1159,
                                                            end: 1160,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1161,
                                                                    end: 1162,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1162,
                                                            end: 1163,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1168,
                                                            end: 1171,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1172,
                                                                end: 1173,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1174,
                                                            end: 1175,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1176,
                                                                    end: 1177,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1177,
                                                            end: 1178,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1183,
                                                            end: 1186,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1187,
                                                                end: 1188,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1189,
                                                            end: 1190,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1191,
                                                                    end: 1192,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1192,
                                                            end: 1193,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1198,
                                                            end: 1201,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1202,
                                                                end: 1203,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1204,
                                                            end: 1205,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1206,
                                                                    end: 1207,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1207,
                                                            end: 1208,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1213,
                                                            end: 1216,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1217,
                                                                end: 1218,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1219,
                                                            end: 1220,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1221,
                                                                    end: 1222,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1222,
                                                            end: 1223,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1228,
                                                            end: 1231,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1232,
                                                                end: 1233,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1234,
                                                            end: 1235,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1236,
                                                                    end: 1237,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1237,
                                                            end: 1238,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1243,
                                                            end: 1246,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1247,
                                                                end: 1248,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1249,
                                                            end: 1250,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1251,
                                                                    end: 1252,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1252,
                                                            end: 1253,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1258,
                                                            end: 1261,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1262,
                                                                end: 1263,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1264,
                                                            end: 1265,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1266,
                                                                    end: 1267,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1267,
                                                            end: 1268,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1273,
                                                            end: 1276,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1277,
                                                                end: 1278,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1279,
                                                            end: 1280,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1281,
                                                                    end: 1282,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1282,
                                                            end: 1283,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1288,
                                                            end: 1291,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1292,
                                                                end: 1293,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1294,
                                                            end: 1295,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1296,
                                                                    end: 1297,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1297,
                                                            end: 1298,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1303,
                                                            end: 1306,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1307,
                                                                end: 1308,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1309,
                                                            end: 1310,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1311,
                                                                    end: 1312,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1312,
                                                            end: 1313,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1318,
                                                            end: 1321,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1322,
                                                                end: 1323,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1324,
                                                            end: 1325,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1326,
                                                                    end: 1327,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1327,
                                                            end: 1328,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1333,
                                                            end: 1336,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1337,
                                                                end: 1338,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1339,
                                                            end: 1340,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1341,
                                                                    end: 1342,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1342,
                                                            end: 1343,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1348,
                                                            end: 1351,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1352,
                                                                end: 1353,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1354,
                                                            end: 1355,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1356,
                                                                    end: 1357,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1357,
                                                            end: 1358,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1363,
                                                            end: 1366,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1367,
                                                                end: 1368,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1369,
                                                            end: 1370,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1371,
                                                                    end: 1372,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1372,
                                                            end: 1373,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1378,
                                                            end: 1381,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1382,
                                                                end: 1383,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1384,
                                                            end: 1385,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1386,
                                                                    end: 1387,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1387,
                                                            end: 1388,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1393,
                                                            end: 1396,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1397,
                                                                end: 1398,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1399,
                                                            end: 1400,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1401,
                                                                    end: 1402,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1402,
                                                            end: 1403,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1408,
                                                            end: 1411,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1412,
                                                                end: 1413,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1414,
                                                            end: 1415,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1416,
                                                                    end: 1417,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1417,
                                                            end: 1418,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1423,
                                                            end: 1426,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1427,
                                                                end: 1428,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1429,
                                                            end: 1430,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1431,
                                                                    end: 1432,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1432,
                                                            end: 1433,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1438,
                                                            end: 1441,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1442,
                                                                end: 1443,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1444,
                                                            end: 1445,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1446,
                                                                    end: 1447,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1447,
                                                            end: 1448,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1453,
                                                            end: 1456,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1457,
                                                                end: 1458,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1459,
                                                            end: 1460,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1461,
                                                                    end: 1462,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1462,
                                                            end: 1463,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1468,
                                                            end: 1471,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1472,
                                                                end: 1473,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1474,
                                                            end: 1475,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1476,
                                                                    end: 1477,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1477,
                                                            end: 1478,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1483,
                                                            end: 1486,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1487,
                                                                end: 1488,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1489,
                                                            end: 1490,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1491,
                                                                    end: 1492,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1492,
                                                            end: 1493,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1498,
                                                            end: 1501,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1502,
                                                                end: 1503,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1504,
                                                            end: 1505,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1506,
                                                                    end: 1507,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1507,
                                                            end: 1508,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1513,
                                                            end: 1516,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1517,
                                                                end: 1518,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1519,
                                                            end: 1520,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1521,
                                                                    end: 1522,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1522,
                                                            end: 1523,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1528,
                                                            end: 1531,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1532,
                                                                end: 1533,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1534,
                                                            end: 1535,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1536,
                                                                    end: 1537,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1537,
                                                            end: 1538,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1543,
                                                            end: 1546,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1547,
                                                                end: 1548,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1549,
                                                            end: 1550,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1551,
                                                                    end: 1552,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1552,
                                                            end: 1553,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1558,
                                                            end: 1561,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1562,
                                                                end: 1563,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1564,
                                                            end: 1565,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1566,
                                                                    end: 1567,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1567,
                                                            end: 1568,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1573,
                                                            end: 1576,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1577,
                                                                end: 1578,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1579,
                                                            end: 1580,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1581,
                                                                    end: 1582,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1582,
                                                            end: 1583,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1588,
                                                            end: 1591,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1592,
                                                                end: 1593,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1594,
                                                            end: 1595,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1596,
                                                                    end: 1597,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1597,
                                                            end: 1598,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1603,
                                                            end: 1606,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1607,
                                                                end: 1608,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1609,
                                                            end: 1610,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1611,
                                                                    end: 1612,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1612,
                                                            end: 1613,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1618,
                                                            end: 1621,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1622,
                                                                end: 1623,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1624,
                                                            end: 1625,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1626,
                                                                    end: 1627,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1627,
                                                            end: 1628,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1633,
                                                            end: 1636,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1637,
                                                                end: 1638,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1639,
                                                            end: 1640,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1641,
                                                                    end: 1642,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1642,
                                                            end: 1643,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1648,
                                                            end: 1651,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1652,
                                                                end: 1653,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1654,
                                                            end: 1655,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1656,
                                                                    end: 1657,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1657,
                                                            end: 1658,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1663,
                                                            end: 1666,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1667,
                                                                end: 1668,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1669,
                                                            end: 1670,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1671,
                                                                    end: 1672,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1672,
                                                            end: 1673,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1678,
                                                            end: 1681,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1682,
                                                                end: 1683,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1684,
                                                            end: 1685,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1686,
                                                                    end: 1687,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1687,
                                                            end: 1688,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1693,
                                                            end: 1696,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1697,
                                                                end: 1698,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1699,
                                                            end: 1700,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1701,
                                                                    end: 1702,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1702,
                                                            end: 1703,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1708,
                                                            end: 1711,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1712,
                                                                end: 1713,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1714,
                                                            end: 1715,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1716,
                                                                    end: 1717,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1717,
                                                            end: 1718,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1723,
                                                            end: 1726,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1727,
                                                                end: 1728,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1729,
                                                            end: 1730,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1731,
                                                                    end: 1732,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1732,
                                                            end: 1733,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1738,
                                                            end: 1741,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1742,
                                                                end: 1743,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1744,
                                                            end: 1745,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1746,
                                                                    end: 1747,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1747,
                                                            end: 1748,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1753,
                                                            end: 1756,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1757,
                                                                end: 1758,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1759,
                                                            end: 1760,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1761,
                                                                    end: 1762,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1762,
                                                            end: 1763,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1768,
                                                            end: 1771,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1772,
                                                                end: 1773,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1774,
                                                            end: 1775,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1776,
                                                                    end: 1777,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1777,
                                                            end: 1778,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1783,
                                                            end: 1786,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1787,
                                                                end: 1788,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1789,
                                                            end: 1790,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1791,
                                                                    end: 1792,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1792,
                                                            end: 1793,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1798,
                                                            end: 1801,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1802,
                                                                end: 1803,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1804,
                                                            end: 1805,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1806,
                                                                    end: 1807,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1807,
                                                            end: 1808,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1813,
                                                            end: 1816,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1817,
                                                                end: 1818,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1819,
                                                            end: 1820,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1821,
                                                                    end: 1822,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1822,
                                                            end: 1823,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1828,
                                                            end: 1831,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1832,
                                                                end: 1833,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1834,
                                                            end: 1835,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1836,
                                                                    end: 1837,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1837,
                                                            end: 1838,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1843,
                                                            end: 1846,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1847,
                                                                end: 1848,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1849,
                                                            end: 1850,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1851,
                                                                    end: 1852,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1852,
                                                            end: 1853,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1858,
                                                            end: 1861,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1862,
                                                                end: 1863,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1864,
                                                            end: 1865,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1866,
                                                                    end: 1867,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1867,
                                                            end: 1868,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1873,
                                                            end: 1876,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1877,
                                                                end: 1878,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1879,
                                                            end: 1880,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1881,
                                                                    end: 1882,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1882,
                                                            end: 1883,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1888,
                                                            end: 1891,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1892,
                                                                end: 1893,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1894,
                                                            end: 1895,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1896,
                                                                    end: 1897,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1897,
                                                            end: 1898,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1903,
                                                            end: 1906,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1907,
                                                                end: 1908,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1909,
                                                            end: 1910,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1911,
                                                                    end: 1912,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1912,
                                                            end: 1913,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1918,
                                                            end: 1921,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1922,
                                                                end: 1923,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1924,
                                                            end: 1925,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1926,
                                                                    end: 1927,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1927,
                                                            end: 1928,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1933,
                                                            end: 1936,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1937,
                                                                end: 1938,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1939,
                                                            end: 1940,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1941,
                                                                    end: 1942,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1942,
                                                            end: 1943,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1948,
                                                            end: 1951,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1952,
                                                                end: 1953,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1954,
                                                            end: 1955,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1956,
                                                                    end: 1957,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1957,
                                                            end: 1958,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1963,
                                                            end: 1966,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1967,
                                                                end: 1968,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1969,
                                                            end: 1970,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1971,
                                                                    end: 1972,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1972,
                                                            end: 1973,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1978,
                                                            end: 1981,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1982,
                                                                end: 1983,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1984,
                                                            end: 1985,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 1986,
                                                                    end: 1987,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1987,
                                                            end: 1988,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1993,
                                                            end: 1996,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 1997,
                                                                end: 1998,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 1999,
                                                            end: 2000,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2001,
                                                                    end: 2002,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2002,
                                                            end: 2003,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2008,
                                                            end: 2011,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2012,
                                                                end: 2013,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2014,
                                                            end: 2015,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2016,
                                                                    end: 2017,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2017,
                                                            end: 2018,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2023,
                                                            end: 2026,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2027,
                                                                end: 2028,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2029,
                                                            end: 2030,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2031,
                                                                    end: 2032,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2032,
                                                            end: 2033,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2038,
                                                            end: 2041,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2042,
                                                                end: 2043,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2044,
                                                            end: 2045,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2046,
                                                                    end: 2047,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2047,
                                                            end: 2048,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2053,
                                                            end: 2056,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2057,
                                                                end: 2058,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2059,
                                                            end: 2060,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2061,
                                                                    end: 2062,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2062,
                                                            end: 2063,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2068,
                                                            end: 2071,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2072,
                                                                end: 2073,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2074,
                                                            end: 2075,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2076,
                                                                    end: 2077,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2077,
                                                            end: 2078,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2083,
                                                            end: 2086,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2087,
                                                                end: 2088,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2089,
                                                            end: 2090,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2091,
                                                                    end: 2092,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2092,
                                                            end: 2093,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2098,
                                                            end: 2101,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2102,
                                                                end: 2103,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2104,
                                                            end: 2105,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2106,
                                                                    end: 2107,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2107,
                                                            end: 2108,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2113,
                                                            end: 2116,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2117,
                                                                end: 2118,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2119,
                                                            end: 2120,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2121,
                                                                    end: 2122,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2122,
                                                            end: 2123,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2128,
                                                            end: 2131,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2132,
                                                                end: 2133,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2134,
                                                            end: 2135,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2136,
                                                                    end: 2137,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2137,
                                                            end: 2138,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2143,
                                                            end: 2146,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2147,
                                                                end: 2148,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2149,
                                                            end: 2150,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2151,
                                                                    end: 2152,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2152,
                                                            end: 2153,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2158,
                                                            end: 2161,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2162,
                                                                end: 2163,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2164,
                                                            end: 2165,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2166,
                                                                    end: 2167,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2167,
                                                            end: 2168,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2173,
                                                            end: 2176,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2177,
                                                                end: 2178,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2179,
                                                            end: 2180,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2181,
                                                                    end: 2182,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2182,
                                                            end: 2183,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2188,
                                                            end: 2191,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2192,
                                                                end: 2193,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2194,
                                                            end: 2195,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2196,
                                                                    end: 2197,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2197,
                                                            end: 2198,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2203,
                                                            end: 2206,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2207,
                                                                end: 2208,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2209,
                                                            end: 2210,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2211,
                                                                    end: 2212,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2212,
                                                            end: 2213,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2218,
                                                            end: 2221,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2222,
                                                                end: 2223,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2224,
                                                            end: 2225,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2226,
                                                                    end: 2227,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2227,
                                                            end: 2228,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2233,
                                                            end: 2236,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2237,
                                                                end: 2238,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2239,
                                                            end: 2240,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2241,
                                                                    end: 2242,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2242,
                                                            end: 2243,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2248,
                                                            end: 2251,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2252,
                                                                end: 2253,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2254,
                                                            end: 2255,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2256,
                                                                    end: 2257,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2257,
                                                            end: 2258,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2263,
                                                            end: 2266,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2267,
                                                                end: 2268,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2269,
                                                            end: 2270,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2271,
                                                                    end: 2272,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2272,
                                                            end: 2273,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2278,
                                                            end: 2281,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2282,
                                                                end: 2283,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2284,
                                                            end: 2285,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2286,
                                                                    end: 2287,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2287,
                                                            end: 2288,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2293,
                                                            end: 2296,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2297,
                                                                end: 2298,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2299,
                                                            end: 2300,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2301,
                                                                    end: 2302,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2302,
                                                            end: 2303,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2308,
                                                            end: 2311,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2312,
                                                                end: 2313,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2314,
                                                            end: 2315,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2316,
                                                                    end: 2317,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2317,
                                                            end: 2318,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2323,
                                                            end: 2326,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2327,
                                                                end: 2328,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2329,
                                                            end: 2330,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2331,
                                                                    end: 2332,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2332,
                                                            end: 2333,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2338,
                                                            end: 2341,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2342,
                                                                end: 2343,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2344,
                                                            end: 2345,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2346,
                                                                    end: 2347,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2347,
                                                            end: 2348,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2353,
                                                            end: 2356,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2357,
                                                                end: 2358,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2359,
                                                            end: 2360,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2361,
                                                                    end: 2362,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2362,
                                                            end: 2363,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2368,
                                                            end: 2371,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2372,
                                                                end: 2373,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2374,
                                                            end: 2375,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2376,
                                                                    end: 2377,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2377,
                                                            end: 2378,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2383,
                                                            end: 2386,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2387,
                                                                end: 2388,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2389,
                                                            end: 2390,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2391,
                                                                    end: 2392,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2392,
                                                            end: 2393,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2398,
                                                            end: 2401,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2402,
                                                                end: 2403,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2404,
                                                            end: 2405,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2406,
                                                                    end: 2407,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2407,
                                                            end: 2408,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2413,
                                                            end: 2416,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2417,
                                                                end: 2418,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2419,
                                                            end: 2420,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2421,
                                                                    end: 2422,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2422,
                                                            end: 2423,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2428,
                                                            end: 2431,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2432,
                                                                end: 2433,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2434,
                                                            end: 2435,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2436,
                                                                    end: 2437,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2437,
                                                            end: 2438,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2443,
                                                            end: 2446,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2447,
                                                                end: 2448,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2449,
                                                            end: 2450,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2451,
                                                                    end: 2452,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2452,
                                                            end: 2453,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2458,
                                                            end: 2461,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2462,
                                                                end: 2463,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2464,
                                                            end: 2465,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2466,
                                                                    end: 2467,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2467,
                                                            end: 2468,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2473,
                                                            end: 2476,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2477,
                                                                end: 2478,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2479,
                                                            end: 2480,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2481,
                                                                    end: 2482,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2482,
                                                            end: 2483,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2488,
                                                            end: 2491,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2492,
                                                                end: 2493,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2494,
                                                            end: 2495,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2496,
                                                                    end: 2497,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2497,
                                                            end: 2498,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2503,
                                                            end: 2506,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2507,
                                                                end: 2508,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2509,
                                                            end: 2510,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2511,
                                                                    end: 2512,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2512,
                                                            end: 2513,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2518,
                                                            end: 2521,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2522,
                                                                end: 2523,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2524,
                                                            end: 2525,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2526,
                                                                    end: 2527,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2527,
                                                            end: 2528,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2533,
                                                            end: 2536,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2537,
                                                                end: 2538,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2539,
                                                            end: 2540,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2541,
                                                                    end: 2542,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2542,
                                                            end: 2543,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2548,
                                                            end: 2551,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2552,
                                                                end: 2553,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2554,
                                                            end: 2555,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2556,
                                                                    end: 2557,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2557,
                                                            end: 2558,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2563,
                                                            end: 2566,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2567,
                                                                end: 2568,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2569,
                                                            end: 2570,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2571,
                                                                    end: 2572,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2572,
                                                            end: 2573,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2578,
                                                            end: 2581,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2582,
                                                                end: 2583,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2584,
                                                            end: 2585,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2586,
                                                                    end: 2587,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2587,
                                                            end: 2588,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2593,
                                                            end: 2596,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2597,
                                                                end: 2598,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2599,
                                                            end: 2600,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2601,
                                                                    end: 2602,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2602,
                                                            end: 2603,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2608,
                                                            end: 2611,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2612,
                                                                end: 2613,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2614,
                                                            end: 2615,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2616,
                                                                    end: 2617,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2617,
                                                            end: 2618,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2623,
                                                            end: 2626,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2627,
                                                                end: 2628,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2629,
                                                            end: 2630,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2631,
                                                                    end: 2632,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2632,
                                                            end: 2633,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2638,
                                                            end: 2641,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2642,
                                                                end: 2643,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2644,
                                                            end: 2645,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2646,
                                                                    end: 2647,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2647,
                                                            end: 2648,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2653,
                                                            end: 2656,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2657,
                                                                end: 2658,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2659,
                                                            end: 2660,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2661,
                                                                    end: 2662,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2662,
                                                            end: 2663,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2668,
                                                            end: 2671,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2672,
                                                                end: 2673,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2674,
                                                            end: 2675,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2676,
                                                                    end: 2677,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2677,
                                                            end: 2678,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2683,
                                                            end: 2686,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2687,
                                                                end: 2688,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2689,
                                                            end: 2690,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2691,
                                                                    end: 2692,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2692,
                                                            end: 2693,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2698,
                                                            end: 2701,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2702,
                                                                end: 2703,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2704,
                                                            end: 2705,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2706,
                                                                    end: 2707,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2707,
                                                            end: 2708,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2713,
                                                            end: 2716,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2717,
                                                                end: 2718,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2719,
                                                            end: 2720,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2721,
                                                                    end: 2722,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2722,
                                                            end: 2723,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2728,
                                                            end: 2731,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2732,
                                                                end: 2733,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2734,
                                                            end: 2735,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2736,
                                                                    end: 2737,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2737,
                                                            end: 2738,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2743,
                                                            end: 2746,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2747,
                                                                end: 2748,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2749,
                                                            end: 2750,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2751,
                                                                    end: 2752,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2752,
                                                            end: 2753,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2758,
                                                            end: 2761,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2762,
                                                                end: 2763,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2764,
                                                            end: 2765,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2766,
                                                                    end: 2767,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2767,
                                                            end: 2768,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2773,
                                                            end: 2776,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2777,
                                                                end: 2778,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2779,
                                                            end: 2780,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2781,
                                                                    end: 2782,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2782,
                                                            end: 2783,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2788,
                                                            end: 2791,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2792,
                                                                end: 2793,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2794,
                                                            end: 2795,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2796,
                                                                    end: 2797,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2797,
                                                            end: 2798,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2803,
                                                            end: 2806,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2807,
                                                                end: 2808,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2809,
                                                            end: 2810,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2811,
                                                                    end: 2812,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2812,
                                                            end: 2813,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2818,
                                                            end: 2821,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 2822,
                                                                end: 2823,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2824,
                                                            end: 2825,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 2826,
                                                                    end: 2827,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 2827,
                                                            end: 2828,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 982,
                                        end: 2830,
                                        as_str(): "{\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
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
