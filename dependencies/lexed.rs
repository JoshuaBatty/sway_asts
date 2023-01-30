Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb118764f50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb118764f50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
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
                                        src (ptr): 0x00007fb118764f50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                        ),
                                        start: 46,
                                        end: 49,
                                        as_str(): "dep",
                                    },
                                },
                                path: DependencyPath {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb118764f50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                            ),
                                            start: 50,
                                            end: 62,
                                            as_str(): "a_dependency",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb118764f50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                        ),
                                        start: 62,
                                        end: 63,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Dependency(
                            Dependency {
                                dep_token: DepToken {
                                    span: Span {
                                        src (ptr): 0x00007fb118764f50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                        ),
                                        start: 64,
                                        end: 67,
                                        as_str(): "dep",
                                    },
                                },
                                path: DependencyPath {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb118764f50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                            ),
                                            start: 68,
                                            end: 85,
                                            as_str(): "nested_dependency",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [
                                        (
                                            ForwardSlashToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb118764f50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                    ),
                                                    start: 85,
                                                    end: 86,
                                                    as_str(): "/",
                                                },
                                            },
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb118764f50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                    ),
                                                    start: 86,
                                                    end: 89,
                                                    as_str(): "bar",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        (
                                            ForwardSlashToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb118764f50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                    ),
                                                    start: 89,
                                                    end: 90,
                                                    as_str(): "/",
                                                },
                                            },
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb118764f50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                    ),
                                                    start: 90,
                                                    end: 93,
                                                    as_str(): "bar",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                    ],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb118764f50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                        ),
                                        start: 93,
                                        end: 94,
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
                                        src (ptr): 0x00007fb118764f50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                        ),
                                        start: 96,
                                        end: 99,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb118764f50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                            ),
                                            start: 100,
                                            end: 103,
                                            as_str(): "foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb118764f50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                            ),
                                            start: 103,
                                            end: 105,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Name {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb118764f50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                ),
                                                start: 105,
                                                end: 108,
                                                as_str(): "Foo",
                                            },
                                            is_raw_ident: false,
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb118764f50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                        ),
                                        start: 108,
                                        end: 109,
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
                                        src (ptr): 0x00007fb118764f50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                        ),
                                        start: 110,
                                        end: 113,
                                        as_str(): "use",
                                    },
                                },
                                root_import: Some(
                                    DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb118764f50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                            ),
                                            start: 114,
                                            end: 116,
                                            as_str(): "::",
                                        },
                                    },
                                ),
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb118764f50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                            ),
                                            start: 116,
                                            end: 119,
                                            as_str(): "foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb118764f50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                            ),
                                            start: 119,
                                            end: 121,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb118764f50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                ),
                                                start: 121,
                                                end: 124,
                                                as_str(): "bar",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb118764f50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                ),
                                                start: 124,
                                                end: 126,
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
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 127,
                                                                        end: 130,
                                                                        as_str(): "Bar",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            CommaToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb118764f50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                    ),
                                                                    start: 130,
                                                                    end: 131,
                                                                    as_str(): ",",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    final_value_opt: Some(
                                                        Path {
                                                            prefix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb118764f50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                    ),
                                                                    start: 132,
                                                                    end: 142,
                                                                    as_str(): "double_bar",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            double_colon_token: DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb118764f50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                    ),
                                                                    start: 142,
                                                                    end: 144,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            suffix: Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 144,
                                                                        end: 153,
                                                                        as_str(): "DoubleBar",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                    ),
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fb118764f50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                    ),
                                                    start: 126,
                                                    end: 154,
                                                    as_str(): "{Bar, double_bar::DoubleBar}",
                                                },
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb118764f50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                        ),
                                        start: 154,
                                        end: 155,
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
                                            src (ptr): 0x00007fb118764f50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                            ),
                                            start: 157,
                                            end: 159,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb118764f50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                            ),
                                            start: 160,
                                            end: 164,
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
                                            src (ptr): 0x00007fb118764f50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                            ),
                                            start: 164,
                                            end: 166,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb118764f50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                    ),
                                                    start: 167,
                                                    end: 169,
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
                                                                src (ptr): 0x00007fb118764f50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                ),
                                                                start: 170,
                                                                end: 174,
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
                                                            src (ptr): 0x00007fb118764f50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                            ),
                                                            start: 181,
                                                            end: 184,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb118764f50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                ),
                                                                start: 185,
                                                                end: 188,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb118764f50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                            ),
                                                            start: 189,
                                                            end: 190,
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
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 191,
                                                                        end: 194,
                                                                        as_str(): "Foo",
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
                                                                                    src (ptr): 0x00007fb118764f50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                    ),
                                                                                    start: 205,
                                                                                    end: 208,
                                                                                    as_str(): "foo",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb118764f50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                            ),
                                                                                            start: 208,
                                                                                            end: 209,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        String(
                                                                                            LitString {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb118764f50,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                                    ),
                                                                                                    start: 210,
                                                                                                    end: 215,
                                                                                                    as_str(): "\"foo\"",
                                                                                                },
                                                                                                parsed: "foo",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb118764f50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                ),
                                                                                start: 215,
                                                                                end: 216,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb118764f50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                ),
                                                                start: 195,
                                                                end: 222,
                                                                as_str(): "{\n        foo: \"foo\",\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb118764f50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                            ),
                                                            start: 222,
                                                            end: 223,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb118764f50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                            ),
                                                            start: 228,
                                                            end: 231,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb118764f50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                ),
                                                                start: 232,
                                                                end: 234,
                                                                as_str(): "db",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb118764f50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                            ),
                                                            start: 235,
                                                            end: 236,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Struct {
                                                        path: PathExpr {
                                                            root_opt: Some(
                                                                (
                                                                    None,
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb118764f50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                            ),
                                                                            start: 237,
                                                                            end: 239,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 239,
                                                                        end: 242,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb118764f50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                            ),
                                                                            start: 242,
                                                                            end: 244,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb118764f50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                ),
                                                                                start: 244,
                                                                                end: 247,
                                                                                as_str(): "bar",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                ),
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb118764f50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                            ),
                                                                            start: 247,
                                                                            end: 249,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb118764f50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                ),
                                                                                start: 249,
                                                                                end: 259,
                                                                                as_str(): "double_bar",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                ),
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb118764f50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                            ),
                                                                            start: 259,
                                                                            end: 261,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb118764f50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                ),
                                                                                start: 261,
                                                                                end: 270,
                                                                                as_str(): "DoubleBar",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                ),
                                                            ],
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
                                                                                    src (ptr): 0x00007fb118764f50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                    ),
                                                                                    start: 281,
                                                                                    end: 282,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb118764f50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                            ),
                                                                                            start: 282,
                                                                                            end: 283,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb118764f50,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                                    ),
                                                                                                    start: 284,
                                                                                                    end: 285,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U32,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fb118764f50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                                            ),
                                                                                                            start: 285,
                                                                                                            end: 288,
                                                                                                            as_str(): "u32",
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb118764f50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                ),
                                                                                start: 288,
                                                                                end: 289,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb118764f50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                ),
                                                                start: 271,
                                                                end: 295,
                                                                as_str(): "{\n        a: 5u32,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb118764f50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                            ),
                                                            start: 295,
                                                            end: 296,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb118764f50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                            ),
                                                            start: 301,
                                                            end: 304,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb118764f50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                ),
                                                                start: 305,
                                                                end: 308,
                                                                as_str(): "bar",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb118764f50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                            ),
                                                            start: 309,
                                                            end: 310,
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
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 311,
                                                                        end: 314,
                                                                        as_str(): "Bar",
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
                                                                                    src (ptr): 0x00007fb118764f50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                    ),
                                                                                    start: 325,
                                                                                    end: 326,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb118764f50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                            ),
                                                                                            start: 326,
                                                                                            end: 327,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb118764f50,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                                    ),
                                                                                                    start: 328,
                                                                                                    end: 329,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U32,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fb118764f50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                                            ),
                                                                                                            start: 329,
                                                                                                            end: 332,
                                                                                                            as_str(): "u32",
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb118764f50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                ),
                                                                                start: 332,
                                                                                end: 333,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb118764f50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                ),
                                                                start: 315,
                                                                end: 339,
                                                                as_str(): "{\n        a: 5u32,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb118764f50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                            ),
                                                            start: 339,
                                                            end: 340,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007fb118764f50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                            ),
                                                            start: 345,
                                                            end: 350,
                                                            as_str(): "false",
                                                        },
                                                        kind: False,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb118764f50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                        ),
                                        start: 175,
                                        end: 352,
                                        as_str(): "{\n    let foo = Foo {\n        foo: \"foo\",\n    };\n    let db = ::foo::bar::double_bar::DoubleBar {\n        a: 5u32,\n    };\n    let bar = Bar {\n        a: 5u32,\n    };\n    false\n}",
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
                            src (ptr): 0x00007fb118a4cac0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                            ),
                            start: 8,
                            end: 11,
                            as_str(): "foo",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb118a4cac0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                ),
                                start: 8,
                                end: 11,
                                as_str(): "foo",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007fb118a4cac0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb118a4cac0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                            ),
                                            start: 8,
                                            end: 11,
                                            as_str(): "foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb118a4cac0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                        ),
                                        start: 11,
                                        end: 12,
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
                                                        src (ptr): 0x00007fb118a4cac0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                        ),
                                                        start: 13,
                                                        end: 16,
                                                        as_str(): "dep",
                                                    },
                                                },
                                                path: DependencyPath {
                                                    prefix: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb118a4cac0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                            ),
                                                            start: 17,
                                                            end: 20,
                                                            as_str(): "bar",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    suffixes: [],
                                                },
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb118a4cac0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                        ),
                                                        start: 20,
                                                        end: 21,
                                                        as_str(): ";",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    Annotated {
                                        attribute_list: [],
                                        value: Dependency(
                                            Dependency {
                                                dep_token: DepToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb118a4cac0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                        ),
                                                        start: 22,
                                                        end: 25,
                                                        as_str(): "dep",
                                                    },
                                                },
                                                path: DependencyPath {
                                                    prefix: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb118a4cac0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                            ),
                                                            start: 26,
                                                            end: 31,
                                                            as_str(): "inner",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    suffixes: [
                                                        (
                                                            ForwardSlashToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb118a4cac0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                                    ),
                                                                    start: 31,
                                                                    end: 32,
                                                                    as_str(): "/",
                                                                },
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb118a4cac0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                                    ),
                                                                    start: 32,
                                                                    end: 35,
                                                                    as_str(): "bar",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                    ],
                                                },
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb118a4cac0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
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
                                        value: Struct(
                                            ItemStruct {
                                                visibility: Some(
                                                    PubToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb118a4cac0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                            ),
                                                            start: 37,
                                                            end: 40,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                struct_token: StructToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb118a4cac0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                        ),
                                                        start: 41,
                                                        end: 47,
                                                        as_str(): "struct",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb118a4cac0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                        ),
                                                        start: 48,
                                                        end: 51,
                                                        as_str(): "Foo",
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
                                                                                src (ptr): 0x00007fb118a4cac0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                                                ),
                                                                                start: 58,
                                                                                end: 61,
                                                                                as_str(): "foo",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        colon_token: ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb118a4cac0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                                                ),
                                                                                start: 61,
                                                                                end: 62,
                                                                                as_str(): ":",
                                                                            },
                                                                        },
                                                                        ty: Str {
                                                                            str_token: StrToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb118a4cac0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                                                    ),
                                                                                    start: 63,
                                                                                    end: 66,
                                                                                    as_str(): "str",
                                                                                },
                                                                            },
                                                                            length: SquareBrackets {
                                                                                inner: Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb118a4cac0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 68,
                                                                                                as_str(): "3",
                                                                                            },
                                                                                            parsed: 3,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb118a4cac0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                                                    ),
                                                                                    start: 66,
                                                                                    end: 69,
                                                                                    as_str(): "[3]",
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118a4cac0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                                        ),
                                                                        start: 69,
                                                                        end: 70,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb118a4cac0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                                        ),
                                                        start: 52,
                                                        end: 72,
                                                        as_str(): "{\n    foo: str[3],\n}",
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
                                            src (ptr): 0x00007fb12b7b4720,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/bar.sw",
                                            ),
                                            start: 8,
                                            end: 11,
                                            as_str(): "bar",
                                        },
                                        is_raw_ident: false,
                                    },
                                    LexedSubmodule {
                                        library_name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb12b7b4720,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/bar.sw",
                                                ),
                                                start: 8,
                                                end: 11,
                                                as_str(): "bar",
                                            },
                                            is_raw_ident: false,
                                        },
                                        module: LexedModule {
                                            tree: Module {
                                                kind: Library {
                                                    library_token: LibraryToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b7b4720,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/bar.sw",
                                                            ),
                                                            start: 0,
                                                            end: 7,
                                                            as_str(): "library",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b7b4720,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/bar.sw",
                                                            ),
                                                            start: 8,
                                                            end: 11,
                                                            as_str(): "bar",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                },
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b7b4720,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/bar.sw",
                                                        ),
                                                        start: 11,
                                                        end: 12,
                                                        as_str(): ";",
                                                    },
                                                },
                                                items: [],
                                            },
                                            submodules: [],
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb118822bc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                            ),
                                            start: 8,
                                            end: 11,
                                            as_str(): "bar",
                                        },
                                        is_raw_ident: false,
                                    },
                                    LexedSubmodule {
                                        library_name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb118822bc0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                ),
                                                start: 8,
                                                end: 11,
                                                as_str(): "bar",
                                            },
                                            is_raw_ident: false,
                                        },
                                        module: LexedModule {
                                            tree: Module {
                                                kind: Library {
                                                    library_token: LibraryToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb118822bc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                            ),
                                                            start: 0,
                                                            end: 7,
                                                            as_str(): "library",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb118822bc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                            ),
                                                            start: 8,
                                                            end: 11,
                                                            as_str(): "bar",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                },
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb118822bc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                        ),
                                                        start: 11,
                                                        end: 12,
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
                                                                        src (ptr): 0x00007fb118822bc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                        ),
                                                                        start: 13,
                                                                        end: 16,
                                                                        as_str(): "dep",
                                                                    },
                                                                },
                                                                path: DependencyPath {
                                                                    prefix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb118822bc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                            ),
                                                                            start: 17,
                                                                            end: 29,
                                                                            as_str(): "double_inner",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    suffixes: [
                                                                        (
                                                                            ForwardSlashToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb118822bc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                                    ),
                                                                                    start: 29,
                                                                                    end: 30,
                                                                                    as_str(): "/",
                                                                                },
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb118822bc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                                    ),
                                                                                    start: 30,
                                                                                    end: 40,
                                                                                    as_str(): "double_bar",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                    ],
                                                                },
                                                                semicolon_token: SemicolonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118822bc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                        ),
                                                                        start: 40,
                                                                        end: 41,
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
                                                                        src (ptr): 0x00007fb118822bc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                        ),
                                                                        start: 43,
                                                                        end: 46,
                                                                        as_str(): "use",
                                                                    },
                                                                },
                                                                root_import: None,
                                                                tree: Path {
                                                                    prefix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb118822bc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                            ),
                                                                            start: 47,
                                                                            end: 57,
                                                                            as_str(): "double_bar",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    double_colon_token: DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb118822bc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                            ),
                                                                            start: 57,
                                                                            end: 59,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    suffix: Name {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb118822bc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                                ),
                                                                                start: 59,
                                                                                end: 68,
                                                                                as_str(): "DoubleBar",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                },
                                                                semicolon_token: SemicolonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118822bc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                        ),
                                                                        start: 68,
                                                                        end: 69,
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
                                                                visibility: Some(
                                                                    PubToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb118822bc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                            ),
                                                                            start: 71,
                                                                            end: 74,
                                                                            as_str(): "pub",
                                                                        },
                                                                    },
                                                                ),
                                                                struct_token: StructToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118822bc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                        ),
                                                                        start: 75,
                                                                        end: 81,
                                                                        as_str(): "struct",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118822bc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                        ),
                                                                        start: 82,
                                                                        end: 85,
                                                                        as_str(): "Bar",
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
                                                                                                src (ptr): 0x00007fb118822bc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                                                ),
                                                                                                start: 92,
                                                                                                end: 93,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        colon_token: ColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb118822bc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
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
                                                                                                            src (ptr): 0x00007fb118822bc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                                                            ),
                                                                                                            start: 95,
                                                                                                            end: 98,
                                                                                                            as_str(): "u32",
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
                                                                                        src (ptr): 0x00007fb118822bc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
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
                                                                        src (ptr): 0x00007fb118822bc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                                                        ),
                                                                        start: 86,
                                                                        end: 101,
                                                                        as_str(): "{\n    a: u32,\n}",
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
                                                            src (ptr): 0x00007fb1084c27b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                                            ),
                                                            start: 8,
                                                            end: 18,
                                                            as_str(): "double_bar",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    LexedSubmodule {
                                                        library_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1084c27b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                                                ),
                                                                start: 8,
                                                                end: 18,
                                                                as_str(): "double_bar",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        module: LexedModule {
                                                            tree: Module {
                                                                kind: Library {
                                                                    library_token: LibraryToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1084c27b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                                                            ),
                                                                            start: 0,
                                                                            end: 7,
                                                                            as_str(): "library",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1084c27b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                                                            ),
                                                                            start: 8,
                                                                            end: 18,
                                                                            as_str(): "double_bar",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                semicolon_token: SemicolonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1084c27b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                                                        ),
                                                                        start: 18,
                                                                        end: 19,
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
                                                                                            src (ptr): 0x00007fb1084c27b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                                                                            ),
                                                                                            start: 39,
                                                                                            end: 42,
                                                                                            as_str(): "pub",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                struct_token: StructToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1084c27b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                                                                        ),
                                                                                        start: 43,
                                                                                        end: 49,
                                                                                        as_str(): "struct",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1084c27b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                                                                        ),
                                                                                        start: 50,
                                                                                        end: 59,
                                                                                        as_str(): "DoubleBar",
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
                                                                                                                src (ptr): 0x00007fb1084c27b0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                                                                                                ),
                                                                                                                start: 66,
                                                                                                                end: 67,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        colon_token: ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1084c27b0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
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
                                                                                                                            src (ptr): 0x00007fb1084c27b0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                                                                                                            ),
                                                                                                                            start: 69,
                                                                                                                            end: 72,
                                                                                                                            as_str(): "u32",
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
                                                                                                        src (ptr): 0x00007fb1084c27b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                                                                                        ),
                                                                                                        start: 72,
                                                                                                        end: 73,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        final_value_opt: None,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1084c27b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                                                                        ),
                                                                                        start: 60,
                                                                                        end: 75,
                                                                                        as_str(): "{\n    a: u32,\n}",
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
                                ),
                            ],
                        },
                    },
                ),
                (
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb108b39d10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                            ),
                            start: 8,
                            end: 11,
                            as_str(): "bar",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb108b39d10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                ),
                                start: 8,
                                end: 11,
                                as_str(): "bar",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007fb108b39d10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb108b39d10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                            ),
                                            start: 8,
                                            end: 11,
                                            as_str(): "bar",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb108b39d10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                        ),
                                        start: 11,
                                        end: 12,
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
                                                        src (ptr): 0x00007fb108b39d10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                        ),
                                                        start: 13,
                                                        end: 16,
                                                        as_str(): "use",
                                                    },
                                                },
                                                root_import: Some(
                                                    DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb108b39d10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                            ),
                                                            start: 17,
                                                            end: 19,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                ),
                                                tree: Path {
                                                    prefix: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb108b39d10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                            ),
                                                            start: 19,
                                                            end: 22,
                                                            as_str(): "foo",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    double_colon_token: DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb108b39d10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                            ),
                                                            start: 22,
                                                            end: 24,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    suffix: Name {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb108b39d10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                                ),
                                                                start: 24,
                                                                end: 27,
                                                                as_str(): "Foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                },
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb108b39d10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                        ),
                                                        start: 27,
                                                        end: 28,
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
                                                visibility: Some(
                                                    PubToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb108b39d10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                            ),
                                                            start: 30,
                                                            end: 33,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                struct_token: StructToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb108b39d10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                        ),
                                                        start: 34,
                                                        end: 40,
                                                        as_str(): "struct",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb108b39d10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                        ),
                                                        start: 41,
                                                        end: 57,
                                                        as_str(): "NestedDependency",
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
                                                                                src (ptr): 0x00007fb108b39d10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                                                ),
                                                                                start: 64,
                                                                                end: 67,
                                                                                as_str(): "num",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        colon_token: ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb108b39d10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
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
                                                                                            src (ptr): 0x00007fb108b39d10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                                                            ),
                                                                                            start: 69,
                                                                                            end: 72,
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
                                                                        src (ptr): 0x00007fb108b39d10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                                        ),
                                                                        start: 72,
                                                                        end: 73,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb108b39d10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/nested_dependency/bar/bar.sw",
                                                        ),
                                                        start: 58,
                                                        end: 75,
                                                        as_str(): "{\n    num: u64,\n}",
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
