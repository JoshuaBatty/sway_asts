Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007f8a1d1ebd20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007f8a1d1ebd20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
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
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
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
                                            src (ptr): 0x00007f8a1d1ebd20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 25,
                                            as_str(): "a_dependency",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 25,
                                        end: 26,
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
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 27,
                                        end: 30,
                                        as_str(): "dep",
                                    },
                                },
                                path: DependencyPath {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1d1ebd20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                            ),
                                            start: 31,
                                            end: 43,
                                            as_str(): "b_dependency",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 44,
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
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 45,
                                        end: 48,
                                        as_str(): "dep",
                                    },
                                },
                                path: DependencyPath {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1d1ebd20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                            ),
                                            start: 49,
                                            end: 61,
                                            as_str(): "c_dependency",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 61,
                                        end: 62,
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
                                            src (ptr): 0x00007f8a1d1ebd20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                            ),
                                            start: 64,
                                            end: 66,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1d1ebd20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                            ),
                                            start: 67,
                                            end: 71,
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
                                            src (ptr): 0x00007f8a1d1ebd20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                            ),
                                            start: 71,
                                            end: 73,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                    ),
                                                    start: 74,
                                                    end: 76,
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
                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                ),
                                                                start: 77,
                                                                end: 80,
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
                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                            ),
                                                            start: 87,
                                                            end: 90,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                ),
                                                                start: 91,
                                                                end: 92,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                            ),
                                                            start: 93,
                                                            end: 94,
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
                                                                        src (ptr): 0x00007f8a1d1ebd20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                        ),
                                                                        start: 95,
                                                                        end: 98,
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
                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                            ),
                                                                            start: 98,
                                                                            end: 100,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                ),
                                                                                start: 100,
                                                                                end: 103,
                                                                                as_str(): "Foo",
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
                                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                    ),
                                                                                    start: 114,
                                                                                    end: 117,
                                                                                    as_str(): "foo",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                            ),
                                                                                            start: 117,
                                                                                            end: 118,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                                    ),
                                                                                                    start: 119,
                                                                                                    end: 120,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U32,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                                            ),
                                                                                                            start: 120,
                                                                                                            end: 123,
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
                                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                ),
                                                                                start: 123,
                                                                                end: 124,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                ),
                                                                start: 104,
                                                                end: 130,
                                                                as_str(): "{\n        foo: 1u32,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                            ),
                                                            start: 130,
                                                            end: 131,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                            ),
                                                            start: 136,
                                                            end: 139,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                ),
                                                                start: 140,
                                                                end: 141,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                            ),
                                                            start: 142,
                                                            end: 143,
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
                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                            ),
                                                                            start: 144,
                                                                            end: 147,
                                                                            as_str(): "bar",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                ),
                                                                                start: 147,
                                                                                end: 149,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                    ),
                                                                                    start: 149,
                                                                                    end: 152,
                                                                                    as_str(): "Bar",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                    ),
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
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
                                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                    ),
                                                                                    start: 154,
                                                                                    end: 157,
                                                                                    as_str(): "Baz",
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
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                    ),
                                                                                    start: 158,
                                                                                    end: 162,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                ),
                                                                start: 157,
                                                                end: 163,
                                                                as_str(): "(true)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                            ),
                                                            start: 163,
                                                            end: 164,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 172,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                ),
                                                                start: 173,
                                                                end: 174,
                                                                as_str(): "z",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 176,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: FuncApp {
                                                        func: Path(
                                                            PathExpr {
                                                                root_opt: Some(
                                                                    (
                                                                        None,
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                ),
                                                                                start: 177,
                                                                                end: 179,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                            ),
                                                                            start: 179,
                                                                            end: 182,
                                                                            as_str(): "bar",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                ),
                                                                                start: 182,
                                                                                end: 184,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                    ),
                                                                                    start: 184,
                                                                                    end: 187,
                                                                                    as_str(): "Bar",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                    ),
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                ),
                                                                                start: 187,
                                                                                end: 189,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                    ),
                                                                                    start: 189,
                                                                                    end: 192,
                                                                                    as_str(): "Baz",
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
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                    ),
                                                                                    start: 193,
                                                                                    end: 198,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                ),
                                                                start: 192,
                                                                end: 199,
                                                                as_str(): "(false)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                            ),
                                                            start: 199,
                                                            end: 200,
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
                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                    ),
                                                                    start: 205,
                                                                    end: 208,
                                                                    as_str(): "baz",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [
                                                            (
                                                                DoubleColonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a1d1ebd20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                        ),
                                                                        start: 208,
                                                                        end: 210,
                                                                        as_str(): "::",
                                                                    },
                                                                },
                                                                PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                            ),
                                                                            start: 210,
                                                                            end: 218,
                                                                            as_str(): "return_1",
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
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1d1ebd20,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                        ),
                                                        start: 218,
                                                        end: 220,
                                                        as_str(): "()",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 81,
                                        end: 222,
                                        as_str(): "{\n    let x = foo::Foo {\n        foo: 1u32,\n    };\n    let y = bar::Bar::Baz(true);\n    let z = ::bar::Bar::Baz(false);\n    baz::return_1()\n}",
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
                            src (ptr): 0x00007f8a124e2460,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
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
                                src (ptr): 0x00007f8a124e2460,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
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
                                            src (ptr): 0x00007f8a124e2460,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a124e2460,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
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
                                        src (ptr): 0x00007f8a124e2460,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
                                        ),
                                        start: 11,
                                        end: 12,
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
                                                            src (ptr): 0x00007f8a124e2460,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
                                                            ),
                                                            start: 14,
                                                            end: 17,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                struct_token: StructToken {
                                                    span: Span {
                                                        src (ptr): 0x00007f8a124e2460,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
                                                        ),
                                                        start: 18,
                                                        end: 24,
                                                        as_str(): "struct",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007f8a124e2460,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
                                                        ),
                                                        start: 25,
                                                        end: 28,
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
                                                                                src (ptr): 0x00007f8a124e2460,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
                                                                                ),
                                                                                start: 35,
                                                                                end: 38,
                                                                                as_str(): "foo",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        colon_token: ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a124e2460,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
                                                                                ),
                                                                                start: 38,
                                                                                end: 39,
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
                                                                                            src (ptr): 0x00007f8a124e2460,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
                                                                                            ),
                                                                                            start: 40,
                                                                                            end: 43,
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
                                                                        src (ptr): 0x00007f8a124e2460,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
                                                                        ),
                                                                        start: 43,
                                                                        end: 44,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a124e2460,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
                                                        ),
                                                        start: 29,
                                                        end: 46,
                                                        as_str(): "{\n    foo: u32,\n}",
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
                (
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a1e1b3da0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
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
                                src (ptr): 0x00007f8a1e1b3da0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
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
                                            src (ptr): 0x00007f8a1e1b3da0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1e1b3da0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
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
                                        src (ptr): 0x00007f8a1e1b3da0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                        ),
                                        start: 11,
                                        end: 12,
                                        as_str(): ";",
                                    },
                                },
                                items: [
                                    Annotated {
                                        attribute_list: [],
                                        value: Enum(
                                            ItemEnum {
                                                visibility: None,
                                                enum_token: EnumToken {
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1e1b3da0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                        ),
                                                        start: 14,
                                                        end: 18,
                                                        as_str(): "enum",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1e1b3da0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                        ),
                                                        start: 19,
                                                        end: 22,
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
                                                                                src (ptr): 0x00007f8a1e1b3da0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                                                ),
                                                                                start: 29,
                                                                                end: 32,
                                                                                as_str(): "Baz",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        colon_token: ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1e1b3da0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
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
                                                                                            src (ptr): 0x00007f8a1e1b3da0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                                                            ),
                                                                                            start: 34,
                                                                                            end: 38,
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
                                                                        src (ptr): 0x00007f8a1e1b3da0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                                        ),
                                                                        start: 38,
                                                                        end: 39,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1e1b3da0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                        ),
                                                        start: 23,
                                                        end: 41,
                                                        as_str(): "{\n    Baz: bool,\n}",
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
                (
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a1933faa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                            ),
                            start: 8,
                            end: 11,
                            as_str(): "baz",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007f8a1933faa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                ),
                                start: 8,
                                end: 11,
                                as_str(): "baz",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a1933faa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1933faa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                            ),
                                            start: 8,
                                            end: 11,
                                            as_str(): "baz",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007f8a1933faa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                        ),
                                        start: 11,
                                        end: 12,
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
                                                            src (ptr): 0x00007f8a1933faa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                                            ),
                                                            start: 14,
                                                            end: 16,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1933faa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                                            ),
                                                            start: 17,
                                                            end: 25,
                                                            as_str(): "return_1",
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
                                                            src (ptr): 0x00007f8a1933faa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                                            ),
                                                            start: 25,
                                                            end: 27,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1933faa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                                                    ),
                                                                    start: 28,
                                                                    end: 30,
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
                                                                                src (ptr): 0x00007f8a1933faa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                                                                ),
                                                                                start: 31,
                                                                                end: 34,
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
                                                    where_clause_opt: None,
                                                },
                                                body: Braces {
                                                    inner: CodeBlockContents {
                                                        statements: [],
                                                        final_expr_opt: Some(
                                                            Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1933faa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                                                            ),
                                                                            start: 41,
                                                                            end: 42,
                                                                            as_str(): "1",
                                                                        },
                                                                        parsed: 1,
                                                                        ty_opt: Some(
                                                                            (
                                                                                U32,
                                                                                Span {
                                                                                    src (ptr): 0x00007f8a1933faa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                                                                    ),
                                                                                    start: 42,
                                                                                    end: 45,
                                                                                    as_str(): "u32",
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1933faa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                                        ),
                                                        start: 35,
                                                        end: 47,
                                                        as_str(): "{\n    1u32\n}",
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
