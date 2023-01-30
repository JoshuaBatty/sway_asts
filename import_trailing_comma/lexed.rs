Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0de482d80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0de482d80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Dependency(
                            Dependency {
                                dep_token: DepToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0de482d80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 12,
                                        as_str(): "dep",
                                    },
                                },
                                path: DependencyPath {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0de482d80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "lib",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0de482d80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 17,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0de482d80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                        ),
                                        start: 18,
                                        end: 21,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0de482d80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                            ),
                                            start: 22,
                                            end: 23,
                                            as_str(): "A",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0de482d80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                            ),
                                            start: 23,
                                            end: 25,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Group {
                                        imports: Braces {
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 26,
                                                                    end: 27,
                                                                    as_str(): "B",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 27,
                                                                end: 28,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 29,
                                                                    end: 30,
                                                                    as_str(): "C",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 30,
                                                                end: 31,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 32,
                                                                    end: 33,
                                                                    as_str(): "D",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 33,
                                                                end: 34,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: None,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0de482d80,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                ),
                                                start: 25,
                                                end: 35,
                                                as_str(): "{B, C, D,}",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0de482d80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                        ),
                                        start: 35,
                                        end: 36,
                                        as_str(): ";",
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
                                            src (ptr): 0x00007fe0de482d80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                            ),
                                            start: 38,
                                            end: 40,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0de482d80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                            ),
                                            start: 41,
                                            end: 45,
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
                                            src (ptr): 0x00007fe0de482d80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                            ),
                                            start: 45,
                                            end: 47,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 48,
                                                    end: 50,
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
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 51,
                                                                end: 54,
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
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 61,
                                                            end: 64,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 65,
                                                                end: 66,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 67,
                                                            end: 68,
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
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 69,
                                                                        end: 70,
                                                                        as_str(): "B",
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
                                                                                    src (ptr): 0x00007fe0de482d80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                    ),
                                                                                    start: 81,
                                                                                    end: 82,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0de482d80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                            ),
                                                                                            start: 82,
                                                                                            end: 83,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0de482d80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                                    ),
                                                                                                    start: 84,
                                                                                                    end: 85,
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
                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                ),
                                                                                start: 85,
                                                                                end: 86,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 71,
                                                                end: 92,
                                                                as_str(): "{\n        b: 0,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 92,
                                                            end: 93,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 101,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 102,
                                                                end: 103,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 104,
                                                            end: 105,
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
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 106,
                                                                        end: 107,
                                                                        as_str(): "C",
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
                                                                                    src (ptr): 0x00007fe0de482d80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                    ),
                                                                                    start: 118,
                                                                                    end: 119,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0de482d80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                            ),
                                                                                            start: 119,
                                                                                            end: 120,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0de482d80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                                    ),
                                                                                                    start: 121,
                                                                                                    end: 122,
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
                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                ),
                                                                                start: 122,
                                                                                end: 123,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 108,
                                                                end: 129,
                                                                as_str(): "{\n        c: 0,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 129,
                                                            end: 130,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 138,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 139,
                                                                end: 140,
                                                                as_str(): "z",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 141,
                                                            end: 142,
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
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 143,
                                                                        end: 144,
                                                                        as_str(): "D",
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
                                                                                    src (ptr): 0x00007fe0de482d80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                    ),
                                                                                    start: 155,
                                                                                    end: 156,
                                                                                    as_str(): "d",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0de482d80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                            ),
                                                                                            start: 156,
                                                                                            end: 157,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0de482d80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                                    ),
                                                                                                    start: 158,
                                                                                                    end: 159,
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
                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                ),
                                                                                start: 159,
                                                                                end: 160,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 145,
                                                                end: 166,
                                                                as_str(): "{\n        d: 0,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 166,
                                                            end: 167,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 175,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 176,
                                                                end: 179,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 180,
                                                            end: 181,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Add {
                                                        lhs: Add {
                                                            lhs: FieldProjection {
                                                                target: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0de482d80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                    ),
                                                                                    start: 182,
                                                                                    end: 183,
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
                                                                dot_token: DotToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 183,
                                                                        end: 184,
                                                                        as_str(): ".",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 184,
                                                                        end: 185,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            add_token: AddToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 186,
                                                                    end: 187,
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
                                                                                    src (ptr): 0x00007fe0de482d80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                    ),
                                                                                    start: 188,
                                                                                    end: 189,
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
                                                                dot_token: DotToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 189,
                                                                        end: 190,
                                                                        as_str(): ".",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 190,
                                                                        end: 191,
                                                                        as_str(): "c",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                        add_token: AddToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 192,
                                                                end: 193,
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
                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                ),
                                                                                start: 194,
                                                                                end: 195,
                                                                                as_str(): "z",
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
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 195,
                                                                    end: 196,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 196,
                                                                    end: 197,
                                                                    as_str(): "d",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 197,
                                                            end: 198,
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
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 203,
                                                                end: 206,
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
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0de482d80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                        ),
                                        start: 55,
                                        end: 208,
                                        as_str(): "{\n    let x = B {\n        b: 0,\n    };\n    let y = C {\n        c: 0,\n    };\n    let z = D {\n        d: 0,\n    };\n    let foo = x.b + y.c + z.d;\n    foo\n}",
                                    },
                                },
                            },
                        ),
                    },
                ],
            },
            submodules: [
                (
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0de9b8690,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                            ),
                            start: 8,
                            end: 9,
                            as_str(): "A",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0de9b8690,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                ),
                                start: 8,
                                end: 9,
                                as_str(): "A",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0de9b8690,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0de9b8690,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                            ),
                                            start: 8,
                                            end: 9,
                                            as_str(): "A",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0de9b8690,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                        ),
                                        start: 9,
                                        end: 10,
                                        as_str(): ";",
                                    },
                                },
                                items: [
                                    Annotated {
                                        attribute_list: [],
                                        value: Struct(
                                            ItemStruct {
                                                visibility: Some(
                                                    PubToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de9b8690,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                            ),
                                                            start: 12,
                                                            end: 15,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                struct_token: StructToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0de9b8690,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                        ),
                                                        start: 16,
                                                        end: 22,
                                                        as_str(): "struct",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0de9b8690,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                        ),
                                                        start: 23,
                                                        end: 24,
                                                        as_str(): "B",
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
                                                                                src (ptr): 0x00007fe0de9b8690,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                                ),
                                                                                start: 31,
                                                                                end: 32,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        colon_token: ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0de9b8690,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                                ),
                                                                                start: 32,
                                                                                end: 33,
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
                                                                                            src (ptr): 0x00007fe0de9b8690,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                                            ),
                                                                                            start: 34,
                                                                                            end: 37,
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
                                                                        src (ptr): 0x00007fe0de9b8690,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                        ),
                                                                        start: 37,
                                                                        end: 38,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0de9b8690,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                        ),
                                                        start: 25,
                                                        end: 40,
                                                        as_str(): "{\n    b: u64,\n}",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    Annotated {
                                        attribute_list: [],
                                        value: Struct(
                                            ItemStruct {
                                                visibility: Some(
                                                    PubToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de9b8690,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                            ),
                                                            start: 41,
                                                            end: 44,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                struct_token: StructToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0de9b8690,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                        ),
                                                        start: 45,
                                                        end: 51,
                                                        as_str(): "struct",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0de9b8690,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                        ),
                                                        start: 52,
                                                        end: 53,
                                                        as_str(): "C",
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
                                                                                src (ptr): 0x00007fe0de9b8690,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                                ),
                                                                                start: 60,
                                                                                end: 61,
                                                                                as_str(): "c",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        colon_token: ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0de9b8690,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                                ),
                                                                                start: 61,
                                                                                end: 62,
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
                                                                                            src (ptr): 0x00007fe0de9b8690,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                                            ),
                                                                                            start: 63,
                                                                                            end: 66,
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
                                                                        src (ptr): 0x00007fe0de9b8690,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                        ),
                                                                        start: 66,
                                                                        end: 67,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0de9b8690,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                        ),
                                                        start: 54,
                                                        end: 69,
                                                        as_str(): "{\n    c: u64,\n}",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    Annotated {
                                        attribute_list: [],
                                        value: Struct(
                                            ItemStruct {
                                                visibility: Some(
                                                    PubToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de9b8690,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                            ),
                                                            start: 70,
                                                            end: 73,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                struct_token: StructToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0de9b8690,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                        ),
                                                        start: 74,
                                                        end: 80,
                                                        as_str(): "struct",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0de9b8690,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                        ),
                                                        start: 81,
                                                        end: 82,
                                                        as_str(): "D",
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
                                                                                src (ptr): 0x00007fe0de9b8690,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                                ),
                                                                                start: 89,
                                                                                end: 90,
                                                                                as_str(): "d",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        colon_token: ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0de9b8690,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                                ),
                                                                                start: 90,
                                                                                end: 91,
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
                                                                                            src (ptr): 0x00007fe0de9b8690,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
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
                                                                    },
                                                                },
                                                                CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0de9b8690,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                        ),
                                                                        start: 95,
                                                                        end: 96,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0de9b8690,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                        ),
                                                        start: 83,
                                                        end: 98,
                                                        as_str(): "{\n    d: u64,\n}",
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
                ),
            ],
        },
    },
)
