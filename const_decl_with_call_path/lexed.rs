Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb110d20660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb110d20660,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
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
                                        src (ptr): 0x00007fb110d20660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
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
                                            src (ptr): 0x00007fb110d20660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 21,
                                            as_str(): "test_lib",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb110d20660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                        ),
                                        start: 21,
                                        end: 22,
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
                                        src (ptr): 0x00007fb110d20660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                        ),
                                        start: 24,
                                        end: 27,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb110d20660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                            ),
                                            start: 28,
                                            end: 31,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb110d20660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                            ),
                                            start: 31,
                                            end: 33,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Group {
                                        imports: Braces {
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        Path {
                                                            prefix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb110d20660,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                    ),
                                                                    start: 34,
                                                                    end: 40,
                                                                    as_str(): "assert",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            double_colon_token: DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb110d20660,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                    ),
                                                                    start: 40,
                                                                    end: 42,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            suffix: Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 42,
                                                                        end: 48,
                                                                        as_str(): "assert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 48,
                                                                end: 49,
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
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 50,
                                                                end: 61,
                                                                as_str(): "contract_id",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        double_colon_token: DoubleColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 61,
                                                                end: 63,
                                                                as_str(): "::",
                                                            },
                                                        },
                                                        suffix: Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb110d20660,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                    ),
                                                                    start: 63,
                                                                    end: 73,
                                                                    as_str(): "ContractId",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    },
                                                ),
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb110d20660,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                ),
                                                start: 33,
                                                end: 74,
                                                as_str(): "{assert::assert, contract_id::ContractId}",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb110d20660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                        ),
                                        start: 74,
                                        end: 75,
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
                                            src (ptr): 0x00007fb110d20660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                            ),
                                            start: 77,
                                            end: 79,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb110d20660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                            ),
                                            start: 80,
                                            end: 84,
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
                                            src (ptr): 0x00007fb110d20660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                            ),
                                            start: 84,
                                            end: 86,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb110d20660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                    ),
                                                    start: 87,
                                                    end: 89,
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
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 90,
                                                                end: 93,
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
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 100,
                                                            end: 103,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 104,
                                                                end: 105,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 106,
                                                            end: 107,
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
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 108,
                                                                        end: 116,
                                                                        as_str(): "test_lib",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 116,
                                                                            end: 118,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb110d20660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                ),
                                                                                start: 118,
                                                                                end: 124,
                                                                                as_str(): "NUMBER",
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
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 124,
                                                            end: 125,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 130,
                                                            end: 133,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 134,
                                                                end: 138,
                                                                as_str(): "zero",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 139,
                                                            end: 140,
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
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 141,
                                                                        end: 144,
                                                                        as_str(): "std",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 144,
                                                                            end: 146,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb110d20660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                ),
                                                                                start: 146,
                                                                                end: 155,
                                                                                as_str(): "constants",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                ),
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 155,
                                                                            end: 157,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb110d20660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                ),
                                                                                start: 157,
                                                                                end: 166,
                                                                                as_str(): "ZERO_B256",
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
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
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
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
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
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 176,
                                                                end: 189,
                                                                as_str(): "base_asset_id",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 190,
                                                            end: 191,
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
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 192,
                                                                        end: 195,
                                                                        as_str(): "std",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 195,
                                                                            end: 197,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb110d20660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                ),
                                                                                start: 197,
                                                                                end: 206,
                                                                                as_str(): "constants",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                ),
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 206,
                                                                            end: 208,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb110d20660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                ),
                                                                                start: 208,
                                                                                end: 221,
                                                                                as_str(): "BASE_ASSET_ID",
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
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 221,
                                                            end: 222,
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
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 227,
                                                                        end: 233,
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
                                                                                        src (ptr): 0x00007fb110d20660,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                        ),
                                                                                        start: 234,
                                                                                        end: 238,
                                                                                        as_str(): "zero",
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
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 239,
                                                                            end: 241,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb110d20660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                    ),
                                                                                    start: 242,
                                                                                    end: 308,
                                                                                    as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 233,
                                                            end: 309,
                                                            as_str(): "(zero == 0x0000000000000000000000000000000000000000000000000000000000000000)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 309,
                                                            end: 310,
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
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 315,
                                                                        end: 321,
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
                                                                                        src (ptr): 0x00007fb110d20660,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                        ),
                                                                                        start: 322,
                                                                                        end: 335,
                                                                                        as_str(): "base_asset_id",
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
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 336,
                                                                            end: 338,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb110d20660,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                            ),
                                                                                            start: 339,
                                                                                            end: 349,
                                                                                            as_str(): "ContractId",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb110d20660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                                ),
                                                                                                start: 349,
                                                                                                end: 351,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb110d20660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                                    ),
                                                                                                    start: 351,
                                                                                                    end: 355,
                                                                                                    as_str(): "from",
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
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb110d20660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                                        ),
                                                                                                        start: 356,
                                                                                                        end: 360,
                                                                                                        as_str(): "zero",
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
                                                                                src (ptr): 0x00007fb110d20660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                ),
                                                                                start: 355,
                                                                                end: 361,
                                                                                as_str(): "(zero)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 321,
                                                            end: 362,
                                                            as_str(): "(base_asset_id == ContractId::from(zero))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 362,
                                                            end: 363,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 368,
                                                                end: 369,
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
                                        src (ptr): 0x00007fb110d20660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                        ),
                                        start: 94,
                                        end: 371,
                                        as_str(): "{\n    let x = test_lib::NUMBER;\n    let zero = std::constants::ZERO_B256;\n    let base_asset_id = std::constants::BASE_ASSET_ID;\n    assert(zero == 0x0000000000000000000000000000000000000000000000000000000000000000);\n    assert(base_asset_id == ContractId::from(zero));\n    x\n}",
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
                            src (ptr): 0x00007fb1106046e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                            ),
                            start: 8,
                            end: 16,
                            as_str(): "test_lib",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb1106046e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                ),
                                start: 8,
                                end: 16,
                                as_str(): "test_lib",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007fb1106046e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1106046e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                            ),
                                            start: 8,
                                            end: 16,
                                            as_str(): "test_lib",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1106046e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                        ),
                                        start: 16,
                                        end: 17,
                                        as_str(): ";",
                                    },
                                },
                                items: [
                                    Annotated {
                                        attribute_list: [],
                                        value: Const(
                                            ItemConst {
                                                visibility: Some(
                                                    PubToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1106046e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                                            ),
                                                            start: 19,
                                                            end: 22,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                const_token: ConstToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb1106046e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                                        ),
                                                        start: 23,
                                                        end: 28,
                                                        as_str(): "const",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb1106046e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                                        ),
                                                        start: 29,
                                                        end: 35,
                                                        as_str(): "NUMBER",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                ty_opt: Some(
                                                    (
                                                        ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1106046e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                                                ),
                                                                start: 35,
                                                                end: 36,
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
                                                                            src (ptr): 0x00007fb1106046e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                                                            ),
                                                                            start: 37,
                                                                            end: 40,
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
                                                        src (ptr): 0x00007fb1106046e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                                        ),
                                                        start: 41,
                                                        end: 42,
                                                        as_str(): "=",
                                                    },
                                                },
                                                expr: Literal(
                                                    Int(
                                                        LitInt {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1106046e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                                                ),
                                                                start: 43,
                                                                end: 45,
                                                                as_str(): "10",
                                                            },
                                                            parsed: 10,
                                                            ty_opt: None,
                                                        },
                                                    ),
                                                ),
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb1106046e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                                        ),
                                                        start: 45,
                                                        end: 46,
                                                        as_str(): ";",
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
