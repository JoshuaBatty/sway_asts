Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0dac0ad90,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
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
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
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
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 20,
                                            as_str(): "context",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
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
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
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
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 26,
                                            end: 31,
                                            as_str(): "asset",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 31,
                                        end: 32,
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
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 33,
                                        end: 36,
                                        as_str(): "dep",
                                    },
                                },
                                path: DependencyPath {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 37,
                                            end: 42,
                                            as_str(): "utils",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 42,
                                        end: 43,
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
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 45,
                                        end: 48,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 49,
                                            end: 56,
                                            as_str(): "context",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 56,
                                            end: 58,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Name {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0dac0ad90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                ),
                                                start: 58,
                                                end: 65,
                                                as_str(): "Context",
                                            },
                                            is_raw_ident: false,
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 65,
                                        end: 66,
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
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 67,
                                        end: 70,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 71,
                                            end: 76,
                                            as_str(): "utils",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 76,
                                            end: 78,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Name {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0dac0ad90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                ),
                                                start: 78,
                                                end: 85,
                                                as_str(): "Wrapper",
                                            },
                                            is_raw_ident: false,
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 85,
                                        end: 86,
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
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 88,
                                            end: 90,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 91,
                                            end: 98,
                                            as_str(): "eq_test",
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
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 98,
                                            end: 100,
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
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
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
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 110,
                                                                end: 112,
                                                                as_str(): "w1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 113,
                                                            end: 114,
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
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 115,
                                                                            end: 122,
                                                                            as_str(): "Wrapper",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                ),
                                                                                start: 122,
                                                                                end: 124,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                    ),
                                                                                    start: 124,
                                                                                    end: 127,
                                                                                    as_str(): "new",
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
                                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                    ),
                                                                                    start: 128,
                                                                                    end: 129,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 127,
                                                                end: 130,
                                                                as_str(): "(3)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
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
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
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
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 139,
                                                                end: 141,
                                                                as_str(): "w2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
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
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 144,
                                                                            end: 151,
                                                                            as_str(): "Wrapper",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                ),
                                                                                start: 151,
                                                                                end: 153,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                    ),
                                                                                    start: 153,
                                                                                    end: 156,
                                                                                    as_str(): "new",
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
                                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                    ),
                                                                                    start: 157,
                                                                                    end: 158,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 156,
                                                                end: 159,
                                                                as_str(): "(3)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 159,
                                                            end: 160,
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
                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                        ),
                                                                        start: 165,
                                                                        end: 171,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 174,
                                                                                        as_str(): "w1",
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
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 175,
                                                                            end: 177,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                        ),
                                                                                        start: 178,
                                                                                        end: 180,
                                                                                        as_str(): "w2",
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
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 171,
                                                            end: 181,
                                                            as_str(): "(w1 == w2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 181,
                                                            end: 182,
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
                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                        ),
                                                                        start: 186,
                                                                        end: 192,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                            ),
                                                                                            start: 193,
                                                                                            end: 195,
                                                                                            as_str(): "w1",
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
                                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                ),
                                                                                start: 195,
                                                                                end: 196,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                ),
                                                                                start: 196,
                                                                                end: 201,
                                                                                as_str(): "asset",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 202,
                                                                            end: 204,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                            ),
                                                                                            start: 205,
                                                                                            end: 207,
                                                                                            as_str(): "w2",
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
                                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                ),
                                                                                start: 207,
                                                                                end: 208,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                ),
                                                                                start: 208,
                                                                                end: 213,
                                                                                as_str(): "asset",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 192,
                                                            end: 214,
                                                            as_str(): "(w1.asset == w2.asset)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 214,
                                                            end: 215,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 101,
                                        end: 217,
                                        as_str(): "{\n   let w1 = Wrapper::new(3);\n   let w2 = Wrapper::new(3);\n\n   assert(w1 == w2);\n   assert(w1.asset == w2.asset);\n}",
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
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 219,
                                            end: 221,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 222,
                                            end: 226,
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
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 226,
                                            end: 228,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0dac0ad90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                    ),
                                                    start: 229,
                                                    end: 231,
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
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 232,
                                                                end: 235,
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
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                        ),
                                                                        start: 241,
                                                                        end: 248,
                                                                        as_str(): "eq_test",
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
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 248,
                                                            end: 250,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 250,
                                                            end: 251,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 256,
                                                            end: 259,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 260,
                                                                end: 261,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 262,
                                                            end: 263,
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
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 264,
                                                                            end: 271,
                                                                            as_str(): "Context",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                ),
                                                                                start: 271,
                                                                                end: 273,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                    ),
                                                                                    start: 273,
                                                                                    end: 276,
                                                                                    as_str(): "foo",
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
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 276,
                                                                end: 278,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 278,
                                                            end: 279,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            FieldProjection {
                                                target: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                    ),
                                                                    start: 283,
                                                                    end: 284,
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
                                                        src (ptr): 0x00007fe0dac0ad90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                        ),
                                                        start: 284,
                                                        end: 285,
                                                        as_str(): ".",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dac0ad90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                        ),
                                                        start: 285,
                                                        end: 294,
                                                        as_str(): "something",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 236,
                                        end: 296,
                                        as_str(): "{\n   eq_test();\n\n   let x = Context::foo();\n   x.something\n}",
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
                            src (ptr): 0x00007fe0db5dcf00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                            ),
                            start: 8,
                            end: 15,
                            as_str(): "context",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0db5dcf00,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                ),
                                start: 8,
                                end: 15,
                                as_str(): "context",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0db5dcf00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0db5dcf00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                            ),
                                            start: 8,
                                            end: 15,
                                            as_str(): "context",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0db5dcf00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                        ),
                                        start: 15,
                                        end: 16,
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
                                                            src (ptr): 0x00007fe0db5dcf00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                            ),
                                                            start: 17,
                                                            end: 20,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                struct_token: StructToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0db5dcf00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                        ),
                                                        start: 21,
                                                        end: 27,
                                                        as_str(): "struct",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0db5dcf00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                        ),
                                                        start: 28,
                                                        end: 35,
                                                        as_str(): "Context",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics: None,
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
                                                                            src (ptr): 0x00007fe0db5dcf00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                            ),
                                                                            start: 40,
                                                                            end: 49,
                                                                            as_str(): "something",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    colon_token: ColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0db5dcf00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                            ),
                                                                            start: 49,
                                                                            end: 50,
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
                                                                                        src (ptr): 0x00007fe0db5dcf00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
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
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0db5dcf00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                        ),
                                                        start: 36,
                                                        end: 56,
                                                        as_str(): "{\n  something: u64\n}",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    Annotated {
                                        attribute_list: [],
                                        value: Impl(
                                            ItemImpl {
                                                impl_token: ImplToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0db5dcf00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                        ),
                                                        start: 58,
                                                        end: 62,
                                                        as_str(): "impl",
                                                    },
                                                },
                                                generic_params_opt: None,
                                                trait_opt: None,
                                                ty: Path(
                                                    PathType {
                                                        root_opt: None,
                                                        prefix: PathTypeSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0db5dcf00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                    ),
                                                                    start: 63,
                                                                    end: 70,
                                                                    as_str(): "Context",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [],
                                                    },
                                                ),
                                                where_clause_opt: None,
                                                contents: Braces {
                                                    inner: [
                                                        Annotated {
                                                            attribute_list: [],
                                                            value: ItemFn {
                                                                fn_signature: FnSignature {
                                                                    visibility: Some(
                                                                        PubToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0db5dcf00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                                ),
                                                                                start: 75,
                                                                                end: 78,
                                                                                as_str(): "pub",
                                                                            },
                                                                        },
                                                                    ),
                                                                    fn_token: FnToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0db5dcf00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                            ),
                                                                            start: 79,
                                                                            end: 81,
                                                                            as_str(): "fn",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0db5dcf00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                            ),
                                                                            start: 82,
                                                                            end: 85,
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
                                                                            src (ptr): 0x00007fe0db5dcf00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                            ),
                                                                            start: 85,
                                                                            end: 87,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                    return_type_opt: Some(
                                                                        (
                                                                            RightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0db5dcf00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                                    ),
                                                                                    start: 88,
                                                                                    end: 90,
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
                                                                                                src (ptr): 0x00007fe0db5dcf00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                                                ),
                                                                                                start: 91,
                                                                                                end: 95,
                                                                                                as_str(): "Self",
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
                                                                                                src (ptr): 0x00007fe0db5dcf00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                                                ),
                                                                                                start: 102,
                                                                                                end: 109,
                                                                                                as_str(): "Context",
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
                                                                                                        src (ptr): 0x00007fe0db5dcf00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                                                        ),
                                                                                                        start: 112,
                                                                                                        end: 121,
                                                                                                        as_str(): "something",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                expr_opt: Some(
                                                                                                    (
                                                                                                        ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0db5dcf00,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                                                                ),
                                                                                                                start: 121,
                                                                                                                end: 122,
                                                                                                                as_str(): ":",
                                                                                                            },
                                                                                                        },
                                                                                                        Literal(
                                                                                                            Int(
                                                                                                                LitInt {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0db5dcf00,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                                                                        ),
                                                                                                                        start: 123,
                                                                                                                        end: 125,
                                                                                                                        as_str(): "10",
                                                                                                                    },
                                                                                                                    parsed: 10,
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
                                                                                        src (ptr): 0x00007fe0db5dcf00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                                        ),
                                                                                        start: 110,
                                                                                        end: 127,
                                                                                        as_str(): "{ something: 10 }",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0db5dcf00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                                        ),
                                                                        start: 96,
                                                                        end: 131,
                                                                        as_str(): "{\n    Context { something: 10 }\n  }",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0db5dcf00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                                        ),
                                                        start: 71,
                                                        end: 133,
                                                        as_str(): "{\n  pub fn foo() -> Self {\n    Context { something: 10 }\n  }\n}",
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
                            src (ptr): 0x00007fe0dabeaf30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                            ),
                            start: 8,
                            end: 13,
                            as_str(): "asset",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0dabeaf30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                ),
                                start: 8,
                                end: 13,
                                as_str(): "asset",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0dabeaf30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dabeaf30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                            ),
                                            start: 8,
                                            end: 13,
                                            as_str(): "asset",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dabeaf30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                        ),
                                        start: 13,
                                        end: 14,
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
                                                        src (ptr): 0x00007fe0dabeaf30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                        ),
                                                        start: 16,
                                                        end: 19,
                                                        as_str(): "use",
                                                    },
                                                },
                                                root_import: None,
                                                tree: Path {
                                                    prefix: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dabeaf30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                            ),
                                                            start: 20,
                                                            end: 24,
                                                            as_str(): "core",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    double_colon_token: DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dabeaf30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                            ),
                                                            start: 24,
                                                            end: 26,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    suffix: Path {
                                                        prefix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dabeaf30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                ),
                                                                start: 26,
                                                                end: 29,
                                                                as_str(): "ops",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        double_colon_token: DoubleColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dabeaf30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                ),
                                                                start: 29,
                                                                end: 31,
                                                                as_str(): "::",
                                                            },
                                                        },
                                                        suffix: Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dabeaf30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                    ),
                                                                    start: 31,
                                                                    end: 33,
                                                                    as_str(): "Eq",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    },
                                                },
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dabeaf30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                        ),
                                                        start: 33,
                                                        end: 34,
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
                                                            src (ptr): 0x00007fe0dabeaf30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                            ),
                                                            start: 36,
                                                            end: 39,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                struct_token: StructToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dabeaf30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                        ),
                                                        start: 40,
                                                        end: 46,
                                                        as_str(): "struct",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dabeaf30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                        ),
                                                        start: 47,
                                                        end: 52,
                                                        as_str(): "Asset",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics: None,
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
                                                                            src (ptr): 0x00007fe0dabeaf30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                            ),
                                                                            start: 59,
                                                                            end: 64,
                                                                            as_str(): "value",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    colon_token: ColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dabeaf30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                            ),
                                                                            start: 64,
                                                                            end: 65,
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
                                                                                        src (ptr): 0x00007fe0dabeaf30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                        ),
                                                                                        start: 66,
                                                                                        end: 69,
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
                                                        src (ptr): 0x00007fe0dabeaf30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                        ),
                                                        start: 53,
                                                        end: 71,
                                                        as_str(): "{\n    value: u64\n}",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    Annotated {
                                        attribute_list: [],
                                        value: Impl(
                                            ItemImpl {
                                                impl_token: ImplToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dabeaf30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                        ),
                                                        start: 73,
                                                        end: 77,
                                                        as_str(): "impl",
                                                    },
                                                },
                                                generic_params_opt: None,
                                                trait_opt: Some(
                                                    (
                                                        PathType {
                                                            root_opt: None,
                                                            prefix: PathTypeSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dabeaf30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                        ),
                                                                        start: 78,
                                                                        end: 80,
                                                                        as_str(): "Eq",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                        },
                                                        ForToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dabeaf30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                ),
                                                                start: 81,
                                                                end: 84,
                                                                as_str(): "for",
                                                            },
                                                        },
                                                    ),
                                                ),
                                                ty: Path(
                                                    PathType {
                                                        root_opt: None,
                                                        prefix: PathTypeSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dabeaf30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                    ),
                                                                    start: 85,
                                                                    end: 90,
                                                                    as_str(): "Asset",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [],
                                                    },
                                                ),
                                                where_clause_opt: None,
                                                contents: Braces {
                                                    inner: [
                                                        Annotated {
                                                            attribute_list: [],
                                                            value: ItemFn {
                                                                fn_signature: FnSignature {
                                                                    visibility: None,
                                                                    fn_token: FnToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dabeaf30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                            ),
                                                                            start: 97,
                                                                            end: 99,
                                                                            as_str(): "fn",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dabeaf30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                            ),
                                                                            start: 100,
                                                                            end: 102,
                                                                            as_str(): "eq",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics: None,
                                                                    arguments: Parens {
                                                                        inner: NonStatic {
                                                                            self_token: SelfToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0dabeaf30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                    ),
                                                                                    start: 103,
                                                                                    end: 107,
                                                                                    as_str(): "self",
                                                                                },
                                                                            },
                                                                            ref_self: None,
                                                                            mutable_self: None,
                                                                            args_opt: Some(
                                                                                (
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0dabeaf30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                            ),
                                                                                            start: 107,
                                                                                            end: 108,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
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
                                                                                                            src (ptr): 0x00007fe0dabeaf30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                                            ),
                                                                                                            start: 109,
                                                                                                            end: 114,
                                                                                                            as_str(): "other",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                                colon_token: ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0dabeaf30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                                        ),
                                                                                                        start: 114,
                                                                                                        end: 115,
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
                                                                                                                    src (ptr): 0x00007fe0dabeaf30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                                                    ),
                                                                                                                    start: 116,
                                                                                                                    end: 120,
                                                                                                                    as_str(): "Self",
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
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dabeaf30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                            ),
                                                                            start: 102,
                                                                            end: 121,
                                                                            as_str(): "(self, other: Self)",
                                                                        },
                                                                    },
                                                                    return_type_opt: Some(
                                                                        (
                                                                            RightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0dabeaf30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
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
                                                                                                src (ptr): 0x00007fe0dabeaf30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                                ),
                                                                                                start: 125,
                                                                                                end: 129,
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
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Equal {
                                                                                lhs: FieldProjection {
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0dabeaf30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                                        ),
                                                                                                        start: 140,
                                                                                                        end: 144,
                                                                                                        as_str(): "self",
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
                                                                                            src (ptr): 0x00007fe0dabeaf30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                            ),
                                                                                            start: 144,
                                                                                            end: 145,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0dabeaf30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                            ),
                                                                                            start: 145,
                                                                                            end: 150,
                                                                                            as_str(): "value",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                double_eq_token: DoubleEqToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0dabeaf30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                        ),
                                                                                        start: 151,
                                                                                        end: 153,
                                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe0dabeaf30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                                        ),
                                                                                                        start: 154,
                                                                                                        end: 159,
                                                                                                        as_str(): "other",
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
                                                                                            src (ptr): 0x00007fe0dabeaf30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                            ),
                                                                                            start: 159,
                                                                                            end: 160,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0dabeaf30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                                            ),
                                                                                            start: 160,
                                                                                            end: 165,
                                                                                            as_str(): "value",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dabeaf30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                                        ),
                                                                        start: 130,
                                                                        end: 171,
                                                                        as_str(): "{\n        self.value == other.value\n    }",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dabeaf30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                        ),
                                                        start: 91,
                                                        end: 173,
                                                        as_str(): "{\n    fn eq(self, other: Self) -> bool {\n        self.value == other.value\n    }\n}",
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
                            src (ptr): 0x00007fe0d90fed50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                            ),
                            start: 8,
                            end: 13,
                            as_str(): "utils",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0d90fed50,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                ),
                                start: 8,
                                end: 13,
                                as_str(): "utils",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0d90fed50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d90fed50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                            ),
                                            start: 8,
                                            end: 13,
                                            as_str(): "utils",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0d90fed50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                        ),
                                        start: 13,
                                        end: 14,
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
                                                        src (ptr): 0x00007fe0d90fed50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                        ),
                                                        start: 16,
                                                        end: 19,
                                                        as_str(): "use",
                                                    },
                                                },
                                                root_import: None,
                                                tree: Path {
                                                    prefix: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d90fed50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                            ),
                                                            start: 20,
                                                            end: 24,
                                                            as_str(): "core",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    double_colon_token: DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d90fed50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                            ),
                                                            start: 24,
                                                            end: 26,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    suffix: Path {
                                                        prefix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d90fed50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                ),
                                                                start: 26,
                                                                end: 29,
                                                                as_str(): "ops",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        double_colon_token: DoubleColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d90fed50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                ),
                                                                start: 29,
                                                                end: 31,
                                                                as_str(): "::",
                                                            },
                                                        },
                                                        suffix: Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                    ),
                                                                    start: 31,
                                                                    end: 33,
                                                                    as_str(): "Eq",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    },
                                                },
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d90fed50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                        ),
                                                        start: 33,
                                                        end: 34,
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
                                                        src (ptr): 0x00007fe0d90fed50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                        ),
                                                        start: 35,
                                                        end: 38,
                                                        as_str(): "use",
                                                    },
                                                },
                                                root_import: Some(
                                                    DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d90fed50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                            ),
                                                            start: 39,
                                                            end: 41,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                ),
                                                tree: Path {
                                                    prefix: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d90fed50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                            ),
                                                            start: 41,
                                                            end: 46,
                                                            as_str(): "asset",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    double_colon_token: DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d90fed50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                            ),
                                                            start: 46,
                                                            end: 48,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    suffix: Name {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d90fed50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                ),
                                                                start: 48,
                                                                end: 53,
                                                                as_str(): "Asset",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                },
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d90fed50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                        ),
                                                        start: 53,
                                                        end: 54,
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
                                                            src (ptr): 0x00007fe0d90fed50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                            ),
                                                            start: 56,
                                                            end: 59,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                struct_token: StructToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d90fed50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                        ),
                                                        start: 60,
                                                        end: 66,
                                                        as_str(): "struct",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d90fed50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                        ),
                                                        start: 67,
                                                        end: 74,
                                                        as_str(): "Wrapper",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics: None,
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
                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                            ),
                                                                            start: 81,
                                                                            end: 86,
                                                                            as_str(): "asset",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    colon_token: ColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                            ),
                                                                            start: 86,
                                                                            end: 87,
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
                                                                                        src (ptr): 0x00007fe0d90fed50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                        ),
                                                                                        start: 88,
                                                                                        end: 93,
                                                                                        as_str(): "Asset",
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
                                                        src (ptr): 0x00007fe0d90fed50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                        ),
                                                        start: 75,
                                                        end: 95,
                                                        as_str(): "{\n    asset: Asset\n}",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    Annotated {
                                        attribute_list: [],
                                        value: Impl(
                                            ItemImpl {
                                                impl_token: ImplToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d90fed50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                        ),
                                                        start: 97,
                                                        end: 101,
                                                        as_str(): "impl",
                                                    },
                                                },
                                                generic_params_opt: None,
                                                trait_opt: None,
                                                ty: Path(
                                                    PathType {
                                                        root_opt: None,
                                                        prefix: PathTypeSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                    ),
                                                                    start: 102,
                                                                    end: 109,
                                                                    as_str(): "Wrapper",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [],
                                                    },
                                                ),
                                                where_clause_opt: None,
                                                contents: Braces {
                                                    inner: [
                                                        Annotated {
                                                            attribute_list: [],
                                                            value: ItemFn {
                                                                fn_signature: FnSignature {
                                                                    visibility: Some(
                                                                        PubToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d90fed50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                ),
                                                                                start: 116,
                                                                                end: 119,
                                                                                as_str(): "pub",
                                                                            },
                                                                        },
                                                                    ),
                                                                    fn_token: FnToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                            ),
                                                                            start: 120,
                                                                            end: 122,
                                                                            as_str(): "fn",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                            ),
                                                                            start: 123,
                                                                            end: 126,
                                                                            as_str(): "new",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics: None,
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
                                                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                    ),
                                                                                                    start: 127,
                                                                                                    end: 132,
                                                                                                    as_str(): "value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        colon_token: ColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d90fed50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
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
                                                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                            ),
                                                                                                            start: 134,
                                                                                                            end: 137,
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
                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                            ),
                                                                            start: 126,
                                                                            end: 138,
                                                                            as_str(): "(value: u64)",
                                                                        },
                                                                    },
                                                                    return_type_opt: Some(
                                                                        (
                                                                            RightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                    ),
                                                                                    start: 139,
                                                                                    end: 141,
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
                                                                                                src (ptr): 0x00007fe0d90fed50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                ),
                                                                                                start: 142,
                                                                                                end: 146,
                                                                                                as_str(): "Self",
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
                                                                                                src (ptr): 0x00007fe0d90fed50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                ),
                                                                                                start: 157,
                                                                                                end: 164,
                                                                                                as_str(): "Wrapper",
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
                                                                                                        src (ptr): 0x00007fe0d90fed50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                        ),
                                                                                                        start: 179,
                                                                                                        end: 184,
                                                                                                        as_str(): "asset",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                expr_opt: Some(
                                                                                                    (
                                                                                                        ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d90fed50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                                ),
                                                                                                                start: 184,
                                                                                                                end: 185,
                                                                                                                as_str(): ":",
                                                                                                            },
                                                                                                        },
                                                                                                        Struct {
                                                                                                            path: PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                                            ),
                                                                                                                            start: 186,
                                                                                                                            end: 191,
                                                                                                                            as_str(): "Asset",
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
                                                                                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                                                    ),
                                                                                                                                    start: 210,
                                                                                                                                    end: 215,
                                                                                                                                    as_str(): "value",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            expr_opt: None,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                                    ),
                                                                                                                    start: 192,
                                                                                                                    end: 229,
                                                                                                                    as_str(): "{\n                value\n            }",
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d90fed50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                        ),
                                                                                        start: 165,
                                                                                        end: 239,
                                                                                        as_str(): "{\n            asset: Asset {\n                value\n            }\n        }",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d90fed50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                        ),
                                                                        start: 147,
                                                                        end: 245,
                                                                        as_str(): "{\n        Wrapper {\n            asset: Asset {\n                value\n            }\n        }\n    }",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d90fed50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                        ),
                                                        start: 110,
                                                        end: 247,
                                                        as_str(): "{\n    pub fn new(value: u64) -> Self {\n        Wrapper {\n            asset: Asset {\n                value\n            }\n        }\n    }\n}",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    Annotated {
                                        attribute_list: [],
                                        value: Impl(
                                            ItemImpl {
                                                impl_token: ImplToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d90fed50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                        ),
                                                        start: 249,
                                                        end: 253,
                                                        as_str(): "impl",
                                                    },
                                                },
                                                generic_params_opt: None,
                                                trait_opt: Some(
                                                    (
                                                        PathType {
                                                            root_opt: None,
                                                            prefix: PathTypeSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d90fed50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                        ),
                                                                        start: 254,
                                                                        end: 256,
                                                                        as_str(): "Eq",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                        },
                                                        ForToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d90fed50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                ),
                                                                start: 257,
                                                                end: 260,
                                                                as_str(): "for",
                                                            },
                                                        },
                                                    ),
                                                ),
                                                ty: Path(
                                                    PathType {
                                                        root_opt: None,
                                                        prefix: PathTypeSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                    ),
                                                                    start: 261,
                                                                    end: 268,
                                                                    as_str(): "Wrapper",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [],
                                                    },
                                                ),
                                                where_clause_opt: None,
                                                contents: Braces {
                                                    inner: [
                                                        Annotated {
                                                            attribute_list: [],
                                                            value: ItemFn {
                                                                fn_signature: FnSignature {
                                                                    visibility: None,
                                                                    fn_token: FnToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                            ),
                                                                            start: 275,
                                                                            end: 277,
                                                                            as_str(): "fn",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                            ),
                                                                            start: 278,
                                                                            end: 280,
                                                                            as_str(): "eq",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics: None,
                                                                    arguments: Parens {
                                                                        inner: NonStatic {
                                                                            self_token: SelfToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                    ),
                                                                                    start: 281,
                                                                                    end: 285,
                                                                                    as_str(): "self",
                                                                                },
                                                                            },
                                                                            ref_self: None,
                                                                            mutable_self: None,
                                                                            args_opt: Some(
                                                                                (
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                            ),
                                                                                            start: 285,
                                                                                            end: 286,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
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
                                                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                            ),
                                                                                                            start: 287,
                                                                                                            end: 292,
                                                                                                            as_str(): "other",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                                colon_token: ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0d90fed50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                        ),
                                                                                                        start: 292,
                                                                                                        end: 293,
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
                                                                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                                    ),
                                                                                                                    start: 294,
                                                                                                                    end: 298,
                                                                                                                    as_str(): "Self",
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
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                            ),
                                                                            start: 280,
                                                                            end: 299,
                                                                            as_str(): "(self, other: Self)",
                                                                        },
                                                                    },
                                                                    return_type_opt: Some(
                                                                        (
                                                                            RightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                    ),
                                                                                    start: 300,
                                                                                    end: 302,
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
                                                                                                src (ptr): 0x00007fe0d90fed50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                ),
                                                                                                start: 303,
                                                                                                end: 307,
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
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Equal {
                                                                                lhs: FieldProjection {
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0d90fed50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                        ),
                                                                                                        start: 318,
                                                                                                        end: 322,
                                                                                                        as_str(): "self",
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
                                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                            ),
                                                                                            start: 322,
                                                                                            end: 323,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                            ),
                                                                                            start: 323,
                                                                                            end: 328,
                                                                                            as_str(): "asset",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                double_eq_token: DoubleEqToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d90fed50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                        ),
                                                                                        start: 329,
                                                                                        end: 331,
                                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe0d90fed50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                                        ),
                                                                                                        start: 332,
                                                                                                        end: 337,
                                                                                                        as_str(): "other",
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
                                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                            ),
                                                                                            start: 337,
                                                                                            end: 338,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d90fed50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                                            ),
                                                                                            start: 338,
                                                                                            end: 343,
                                                                                            as_str(): "asset",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d90fed50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                        ),
                                                                        start: 308,
                                                                        end: 349,
                                                                        as_str(): "{\n        self.asset == other.asset\n    }",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d90fed50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                        ),
                                                        start: 269,
                                                        end: 351,
                                                        as_str(): "{\n    fn eq(self, other: Self) -> bool {\n        self.asset == other.asset\n    }\n}",
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
