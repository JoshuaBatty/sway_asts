Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0a2c66060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0a2c66060,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
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
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 24,
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
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 27,
                                                                end: 28,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
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
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 30,
                                                                            end: 33,
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
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 33,
                                                        end: 34,
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
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 35,
                                                            end: 36,
                                                            as_str(): "b",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 36,
                                                            end: 37,
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
                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                        ),
                                                                        start: 38,
                                                                        end: 41,
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
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 25,
                                        end: 43,
                                        as_str(): "{ a: u64, b: u64 }",
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
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 45,
                                        end: 49,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 50,
                                        end: 56,
                                        as_str(): "MyEnum",
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
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 61,
                                                                end: 69,
                                                                as_str(): "Variant1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 69,
                                                                end: 70,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                    ),
                                                                    start: 71,
                                                                    end: 73,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 73,
                                                        end: 74,
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
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 77,
                                                                end: 85,
                                                                as_str(): "Variant2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 85,
                                                                end: 86,
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
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
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
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 90,
                                                        end: 91,
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
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 94,
                                                                end: 102,
                                                                as_str(): "Variant3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 102,
                                                                end: 103,
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
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 104,
                                                                            end: 112,
                                                                            as_str(): "MyStruct",
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
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 112,
                                                        end: 113,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 57,
                                        end: 115,
                                        as_str(): "{\n  Variant1: (),\n  Variant2: u64,\n  Variant3: MyStruct,\n}",
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
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 117,
                                            end: 119,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 120,
                                            end: 124,
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
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 124,
                                            end: 126,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 127,
                                                    end: 129,
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
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 130,
                                                                end: 133,
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
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 138,
                                                            end: 141,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 142,
                                                                end: 143,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 144,
                                                            end: 145,
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
                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                        ),
                                                                        start: 146,
                                                                        end: 152,
                                                                        as_str(): "MyEnum",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 152,
                                                                            end: 154,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                ),
                                                                                start: 154,
                                                                                end: 162,
                                                                                as_str(): "Variant1",
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
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 162,
                                                            end: 163,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 166,
                                                            end: 169,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 170,
                                                                end: 171,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 173,
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
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 174,
                                                                            end: 180,
                                                                            as_str(): "MyEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                ),
                                                                                start: 180,
                                                                                end: 182,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                    ),
                                                                                    start: 182,
                                                                                    end: 190,
                                                                                    as_str(): "Variant2",
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
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                    ),
                                                                                    start: 193,
                                                                                    end: 194,
                                                                                    as_str(): "5",
                                                                                },
                                                                                parsed: 5,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 191,
                                                                end: 196,
                                                                as_str(): "( 5 )",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 197,
                                                            end: 198,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 201,
                                                            end: 204,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 205,
                                                                end: 206,
                                                                as_str(): "z",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 207,
                                                            end: 208,
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
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 209,
                                                                            end: 215,
                                                                            as_str(): "MyEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                ),
                                                                                start: 215,
                                                                                end: 217,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                    ),
                                                                                    start: 217,
                                                                                    end: 225,
                                                                                    as_str(): "Variant3",
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
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 228,
                                                                                        end: 236,
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
                                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
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
                                                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
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
                                                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 242,
                                                                                                                    end: 243,
                                                                                                                    as_str(): "0",
                                                                                                                },
                                                                                                                parsed: 0,
                                                                                                                ty_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                ),
                                                                                                start: 243,
                                                                                                end: 244,
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
                                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                ),
                                                                                                start: 245,
                                                                                                end: 246,
                                                                                                as_str(): "b",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        expr_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                        ),
                                                                                                        start: 246,
                                                                                                        end: 247,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                                ),
                                                                                                                start: 248,
                                                                                                                end: 249,
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
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                ),
                                                                                start: 237,
                                                                                end: 251,
                                                                                as_str(): "{ a: 0, b: 1 }",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 226,
                                                                end: 253,
                                                                as_str(): "( MyStruct { a: 0, b: 1 } )",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 254,
                                                            end: 255,
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
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 259,
                                                        end: 264,
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
                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                    ),
                                                                    start: 265,
                                                                    end: 266,
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
                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                ),
                                                                                start: 273,
                                                                                end: 279,
                                                                                as_str(): "MyEnum",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                    ),
                                                                                    start: 279,
                                                                                    end: 281,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 281,
                                                                                        end: 289,
                                                                                        as_str(): "Variant2",
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
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 292,
                                                                                        end: 293,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                        ),
                                                                        start: 290,
                                                                        end: 295,
                                                                        as_str(): "( y )",
                                                                    },
                                                                },
                                                            },
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                    ),
                                                                    start: 296,
                                                                    end: 298,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                    ),
                                                                                    start: 299,
                                                                                    end: 300,
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
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                        ),
                                                                        start: 300,
                                                                        end: 301,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        MatchBranch {
                                                            pattern: Wildcard {
                                                                underscore_token: UnderscoreToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                        ),
                                                                        start: 306,
                                                                        end: 307,
                                                                        as_str(): "_",
                                                                    },
                                                                },
                                                            },
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                    ),
                                                                    start: 308,
                                                                    end: 310,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                ),
                                                                                start: 311,
                                                                                end: 313,
                                                                                as_str(): "10",
                                                                            },
                                                                            parsed: 10,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                        ),
                                                                        start: 313,
                                                                        end: 314,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 267,
                                                        end: 318,
                                                        as_str(): "{\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 134,
                                        end: 320,
                                        as_str(): "{\n  let x = MyEnum::Variant1;\n  let y = MyEnum::Variant2 ( 5 ) ;\n  let z = MyEnum::Variant3 ( MyStruct { a: 0, b: 1 } ) ;\n\n  match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }\n}",
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
