Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe092b73f20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe092b73f20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
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
                                        src (ptr): 0x00007fe092b73f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe092b73f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
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
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 28,
                                                                end: 29,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 29,
                                                                end: 30,
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
                                                                            src (ptr): 0x00007fe092b73f20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                            ),
                                                                            start: 31,
                                                                            end: 34,
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
                                                        src (ptr): 0x00007fe092b73f20,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                        ),
                                                        start: 34,
                                                        end: 35,
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
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 40,
                                                            end: 41,
                                                            as_str(): "y",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
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
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 43,
                                                                        end: 46,
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
                                        src (ptr): 0x00007fe092b73f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                        ),
                                        start: 22,
                                        end: 48,
                                        as_str(): "{\n    x: u64,\n    y: u64\n}",
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
                                        src (ptr): 0x00007fe092b73f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                        ),
                                        start: 50,
                                        end: 56,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe092b73f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                        ),
                                        start: 57,
                                        end: 61,
                                        as_str(): "Data",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 61,
                                                    end: 62,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 62,
                                                            end: 63,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 63,
                                                    end: 64,
                                                    as_str(): ">",
                                                },
                                            },
                                        },
                                    },
                                ),
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Annotated {
                                                attribute_list: [],
                                                value: TypeField {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 71,
                                                            end: 76,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
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
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 78,
                                                                        end: 79,
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
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe092b73f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                        ),
                                        start: 65,
                                        end: 81,
                                        as_str(): "{\n    value: T\n}",
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
                                            src (ptr): 0x00007fe092b73f20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                            ),
                                            start: 83,
                                            end: 85,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe092b73f20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                            ),
                                            start: 86,
                                            end: 90,
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
                                            src (ptr): 0x00007fe092b73f20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                            ),
                                            start: 90,
                                            end: 92,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 93,
                                                    end: 95,
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
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 96,
                                                                end: 99,
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
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 106,
                                                            end: 109,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 110,
                                                                end: 111,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 112,
                                                            end: 113,
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
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 114,
                                                                        end: 119,
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
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 130,
                                                                                    end: 131,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 131,
                                                                                            end: 132,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 133,
                                                                                                    end: 134,
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
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 134,
                                                                                end: 135,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 144,
                                                                                    end: 145,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 145,
                                                                                            end: 146,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 147,
                                                                                                    end: 148,
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
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 148,
                                                                                end: 149,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 120,
                                                                end: 155,
                                                                as_str(): "{\n        x: 3,\n        y: 4,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 155,
                                                            end: 156,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 161,
                                                            end: 164,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 165,
                                                                end: 166,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 167,
                                                            end: 168,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 169,
                                                                end: 174,
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
                                                                            src (ptr): 0x00007fe092b73f20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                            ),
                                                                            start: 175,
                                                                            end: 176,
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
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 187,
                                                                                        end: 192,
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
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 195,
                                                                                                    end: 196,
                                                                                                    as_str(): "x",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            pattern_opt: Some(
                                                                                                (
                                                                                                    ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 196,
                                                                                                            end: 197,
                                                                                                            as_str(): ":",
                                                                                                        },
                                                                                                    },
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 198,
                                                                                                                    end: 199,
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
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 199,
                                                                                                end: 200,
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
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 201,
                                                                                                end: 202,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        pattern_opt: None,
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 193,
                                                                                end: 204,
                                                                                as_str(): "{ x: 3, y }",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092b73f20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                            ),
                                                                            start: 205,
                                                                            end: 207,
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
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 210,
                                                                                                        end: 211,
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
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 208,
                                                                                end: 213,
                                                                                as_str(): "{ y }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 213,
                                                                                    end: 214,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Struct {
                                                                        path: PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 223,
                                                                                        end: 228,
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
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 231,
                                                                                                    end: 232,
                                                                                                    as_str(): "x",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            pattern_opt: Some(
                                                                                                (
                                                                                                    ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 232,
                                                                                                            end: 233,
                                                                                                            as_str(): ":",
                                                                                                        },
                                                                                                    },
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 234,
                                                                                                                    end: 235,
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
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 235,
                                                                                                end: 236,
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
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 237,
                                                                                                end: 238,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        pattern_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 238,
                                                                                                        end: 239,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 240,
                                                                                                                end: 241,
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
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 229,
                                                                                end: 243,
                                                                                as_str(): "{ x: 3, y: 4 }",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092b73f20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                            ),
                                                                            start: 244,
                                                                            end: 246,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 249,
                                                                                                    end: 251,
                                                                                                    as_str(): "24",
                                                                                                },
                                                                                                parsed: 24,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 247,
                                                                                end: 253,
                                                                                as_str(): "{ 24 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 253,
                                                                                    end: 254,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Wildcard {
                                                                        underscore_token: UnderscoreToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 263,
                                                                                end: 264,
                                                                                as_str(): "_",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092b73f20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                            ),
                                                                            start: 265,
                                                                            end: 267,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 270,
                                                                                                    end: 272,
                                                                                                    as_str(): "24",
                                                                                                },
                                                                                                parsed: 24,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 268,
                                                                                end: 274,
                                                                                as_str(): "{ 24 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 274,
                                                                                    end: 275,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 177,
                                                                end: 281,
                                                                as_str(): "{\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 281,
                                                            end: 282,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 288,
                                                            end: 291,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 292,
                                                                end: 293,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 294,
                                                            end: 295,
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
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 296,
                                                                        end: 300,
                                                                        as_str(): "Data",
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
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 311,
                                                                                end: 316,
                                                                                as_str(): "value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 316,
                                                                                        end: 317,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 318,
                                                                                                end: 322,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                            kind: True,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 301,
                                                                end: 328,
                                                                as_str(): "{\n        value: true\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 328,
                                                            end: 329,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 334,
                                                            end: 337,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 338,
                                                                end: 339,
                                                                as_str(): "d",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 340,
                                                            end: 341,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 342,
                                                                end: 347,
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
                                                                            src (ptr): 0x00007fe092b73f20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                            ),
                                                                            start: 348,
                                                                            end: 349,
                                                                            as_str(): "c",
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
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 360,
                                                                                        end: 364,
                                                                                        as_str(): "Data",
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
                                                                                    Field {
                                                                                        field_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 367,
                                                                                                end: 372,
                                                                                                as_str(): "value",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        pattern_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 372,
                                                                                                        end: 373,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Literal(
                                                                                                    Bool(
                                                                                                        LitBool {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 374,
                                                                                                                end: 379,
                                                                                                                as_str(): "false",
                                                                                                            },
                                                                                                            kind: False,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 365,
                                                                                end: 381,
                                                                                as_str(): "{ value: false }",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092b73f20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                            ),
                                                                            start: 382,
                                                                            end: 384,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 387,
                                                                                                    end: 388,
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
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 385,
                                                                                end: 390,
                                                                                as_str(): "{ 0 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 390,
                                                                                    end: 391,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Struct {
                                                                        path: PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 400,
                                                                                        end: 404,
                                                                                        as_str(): "Data",
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
                                                                                    Field {
                                                                                        field_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 407,
                                                                                                end: 412,
                                                                                                as_str(): "value",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        pattern_opt: None,
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 405,
                                                                                end: 414,
                                                                                as_str(): "{ value }",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092b73f20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                            ),
                                                                            start: 415,
                                                                            end: 417,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 420,
                                                                                                    end: 421,
                                                                                                    as_str(): "4",
                                                                                                },
                                                                                                parsed: 4,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 418,
                                                                                end: 423,
                                                                                as_str(): "{ 4 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 423,
                                                                                    end: 424,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 350,
                                                                end: 430,
                                                                as_str(): "{\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 430,
                                                            end: 431,
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
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 437,
                                                                end: 438,
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
                                        src (ptr): 0x00007fe092b73f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                        ),
                                        start: 100,
                                        end: 440,
                                        as_str(): "{\n    let a = Point {\n        x: 3,\n        y: 4,\n    };\n    let b = match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    };\n\n    let c = Data {\n        value: true\n    };\n    let d = match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    };\n\n    d\n}",
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
