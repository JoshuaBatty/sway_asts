Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe04b383550,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe04b383550,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
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
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 42,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 43,
                                            end: 57,
                                            as_str(): "gimme_a_struct",
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
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 57,
                                            end: 59,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 60,
                                                    end: 62,
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
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 63,
                                                                end: 68,
                                                                as_str(): "Dummy",
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
                                            Struct {
                                                path: PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 75,
                                                                end: 80,
                                                                as_str(): "Dummy",
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
                                                                            src (ptr): 0x00007fe04b383550,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 83,
                                                                            end: 89,
                                                                            as_str(): "value1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    expr_opt: Some(
                                                                        (
                                                                            ColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04b383550,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 89,
                                                                                    end: 90,
                                                                                    as_str(): ":",
                                                                                },
                                                                            },
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04b383550,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 91,
                                                                                            end: 92,
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
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 92,
                                                                        end: 93,
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
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 94,
                                                                        end: 100,
                                                                        as_str(): "value2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                expr_opt: Some(
                                                                    (
                                                                        ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04b383550,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 100,
                                                                                end: 101,
                                                                                as_str(): ":",
                                                                            },
                                                                        },
                                                                        Literal(
                                                                            Bool(
                                                                                LitBool {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04b383550,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                        ),
                                                                                        start: 102,
                                                                                        end: 106,
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
                                                        src (ptr): 0x00007fe04b383550,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                        ),
                                                        start: 81,
                                                        end: 108,
                                                        as_str(): "{ value1: 1, value2: true }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 69,
                                        end: 110,
                                        as_str(): "{\n    Dummy { value1: 1, value2: true }\n}",
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
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 112,
                                            end: 114,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 115,
                                            end: 119,
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
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 119,
                                            end: 121,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 122,
                                                    end: 124,
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
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
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
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 138,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Struct {
                                                        path: PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 139,
                                                                        end: 144,
                                                                        as_str(): "Dummy",
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
                                                                                    src (ptr): 0x00007fe04b383550,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 147,
                                                                                    end: 153,
                                                                                    as_str(): "value1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            pattern_opt: None,
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04b383550,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 153,
                                                                                end: 154,
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
                                                                                src (ptr): 0x00007fe04b383550,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 155,
                                                                                end: 161,
                                                                                as_str(): "value2",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        pattern_opt: None,
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 145,
                                                                end: 163,
                                                                as_str(): "{ value1, value2 }",
                                                            },
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 164,
                                                            end: 165,
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
                                                                            src (ptr): 0x00007fe04b383550,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 166,
                                                                            end: 180,
                                                                            as_str(): "gimme_a_struct",
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
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 180,
                                                                end: 182,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 182,
                                                            end: 183,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 188,
                                                            end: 191,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Struct {
                                                        path: PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 192,
                                                                        end: 197,
                                                                        as_str(): "Dummy",
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
                                                                                    src (ptr): 0x00007fe04b383550,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 200,
                                                                                    end: 206,
                                                                                    as_str(): "value1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            pattern_opt: None,
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04b383550,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 206,
                                                                                end: 207,
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
                                                                                src (ptr): 0x00007fe04b383550,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 208,
                                                                                end: 214,
                                                                                as_str(): "value2",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        pattern_opt: None,
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 198,
                                                                end: 216,
                                                                as_str(): "{ value1, value2 }",
                                                            },
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04b383550,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
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
                                                                                src (ptr): 0x00007fe04b383550,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 218,
                                                                                end: 223,
                                                                                as_str(): "Dummy",
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
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 224,
                                                            end: 225,
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
                                                                            src (ptr): 0x00007fe04b383550,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 226,
                                                                            end: 240,
                                                                            as_str(): "gimme_a_struct",
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
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 240,
                                                                end: 242,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 242,
                                                            end: 243,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 248,
                                                            end: 251,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 252,
                                                                end: 256,
                                                                as_str(): "data",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 257,
                                                            end: 258,
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
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 259,
                                                                        end: 263,
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
                                                                value_separator_pairs: [
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04b383550,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 274,
                                                                                    end: 279,
                                                                                    as_str(): "value",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04b383550,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 279,
                                                                                            end: 280,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04b383550,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 281,
                                                                                                    end: 283,
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
                                                                                src (ptr): 0x00007fe04b383550,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 283,
                                                                                end: 284,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 264,
                                                                end: 290,
                                                                as_str(): "{\n        value: 42,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 290,
                                                            end: 291,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 296,
                                                            end: 299,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Struct {
                                                        path: PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 300,
                                                                        end: 304,
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
                                                                                src (ptr): 0x00007fe04b383550,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 307,
                                                                                end: 312,
                                                                                as_str(): "value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        pattern_opt: None,
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 305,
                                                                end: 314,
                                                                as_str(): "{ value }",
                                                            },
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04b383550,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 314,
                                                                    end: 315,
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
                                                                                src (ptr): 0x00007fe04b383550,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 316,
                                                                                end: 320,
                                                                                as_str(): "Data",
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
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 321,
                                                            end: 322,
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
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 323,
                                                                        end: 327,
                                                                        as_str(): "data",
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
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 327,
                                                            end: 328,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 333,
                                                            end: 339,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04b383550,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 340,
                                                                            end: 345,
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
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 345,
                                                            end: 346,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 129,
                                        end: 348,
                                        as_str(): "{\n    let Dummy { value1, value2 } = gimme_a_struct();\n    let Dummy { value1, value2 }: Dummy = gimme_a_struct();\n    let data = Data {\n        value: 42,\n    };\n    let Data { value }: Data = data;\n    return value;\n}",
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
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 350,
                                        end: 356,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 357,
                                        end: 361,
                                        as_str(): "Data",
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
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 369,
                                                                end: 374,
                                                                as_str(): "value",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 374,
                                                                end: 375,
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
                                                                            src (ptr): 0x00007fe04b383550,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 376,
                                                                            end: 379,
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
                                                        src (ptr): 0x00007fe04b383550,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                        ),
                                                        start: 379,
                                                        end: 380,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 362,
                                        end: 382,
                                        as_str(): "{ \n    value: u64,\n}",
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
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 384,
                                        end: 390,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 391,
                                        end: 396,
                                        as_str(): "Dummy",
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
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 403,
                                                                end: 409,
                                                                as_str(): "value1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 409,
                                                                end: 410,
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
                                                                            src (ptr): 0x00007fe04b383550,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 411,
                                                                            end: 414,
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
                                                        src (ptr): 0x00007fe04b383550,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                        ),
                                                        start: 414,
                                                        end: 415,
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
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 420,
                                                                end: 426,
                                                                as_str(): "value2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 426,
                                                                end: 427,
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
                                                                            src (ptr): 0x00007fe04b383550,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 428,
                                                                            end: 432,
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
                                                        src (ptr): 0x00007fe04b383550,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                        ),
                                                        start: 432,
                                                        end: 433,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 397,
                                        end: 435,
                                        as_str(): "{\n    value1: u64,\n    value2: bool,\n}",
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
