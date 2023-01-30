Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb0e6588390,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
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
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
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
                                                src (ptr): 0x00007fb0e6588390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 25,
                                                as_str(): "address",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb0e6588390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                ),
                                                start: 25,
                                                end: 27,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e6588390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                    ),
                                                    start: 27,
                                                    end: 34,
                                                    as_str(): "Address",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 34,
                                        end: 35,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 36,
                                        end: 39,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 43,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 43,
                                            end: 45,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0e6588390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                ),
                                                start: 45,
                                                end: 51,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb0e6588390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                ),
                                                start: 51,
                                                end: 53,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e6588390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                    ),
                                                    start: 53,
                                                    end: 59,
                                                    as_str(): "assert",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 59,
                                        end: 60,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 61,
                                        end: 64,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 65,
                                            end: 68,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 68,
                                            end: 70,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0e6588390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                ),
                                                start: 70,
                                                end: 74,
                                                as_str(): "b512",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb0e6588390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                ),
                                                start: 74,
                                                end: 76,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e6588390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                    ),
                                                    start: 76,
                                                    end: 80,
                                                    as_str(): "B512",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 80,
                                        end: 81,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 82,
                                        end: 85,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 86,
                                            end: 89,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 89,
                                            end: 91,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0e6588390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                ),
                                                start: 91,
                                                end: 102,
                                                as_str(): "contract_id",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb0e6588390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                ),
                                                start: 102,
                                                end: 104,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e6588390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                    ),
                                                    start: 104,
                                                    end: 114,
                                                    as_str(): "ContractId",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 114,
                                        end: 115,
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
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 117,
                                            end: 119,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
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
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
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
                                                    src (ptr): 0x00007fb0e6588390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
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
                                                                src (ptr): 0x00007fb0e6588390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                ),
                                                                start: 130,
                                                                end: 134,
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
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 164,
                                                                        end: 170,
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
                                                                    lhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 171,
                                                                                    end: 172,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 172,
                                                                                            end: 174,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 175,
                                                                            end: 177,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 178,
                                                                                    end: 179,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 179,
                                                                                            end: 181,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 170,
                                                            end: 182,
                                                            as_str(): "(1u8 == 1u8)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 182,
                                                            end: 183,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 188,
                                                                        end: 194,
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
                                                                NotEqual {
                                                                    lhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 195,
                                                                                    end: 196,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 196,
                                                                                            end: 198,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    bang_eq_token: BangEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 199,
                                                                            end: 201,
                                                                            as_str(): "!=",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 202,
                                                                                    end: 203,
                                                                                    as_str(): "2",
                                                                                },
                                                                                parsed: 2,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 203,
                                                                                            end: 205,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 206,
                                                            as_str(): "(1u8 != 2u8)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 206,
                                                            end: 207,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 213,
                                                                        end: 219,
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
                                                                    lhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 220,
                                                                                    end: 221,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U16,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 221,
                                                                                            end: 224,
                                                                                            as_str(): "u16",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 225,
                                                                            end: 227,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 228,
                                                                                    end: 229,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U16,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 229,
                                                                                            end: 232,
                                                                                            as_str(): "u16",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 219,
                                                            end: 233,
                                                            as_str(): "(1u16 == 1u16)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 233,
                                                            end: 234,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 239,
                                                                        end: 245,
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
                                                                NotEqual {
                                                                    lhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 246,
                                                                                    end: 247,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U16,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 247,
                                                                                            end: 250,
                                                                                            as_str(): "u16",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    bang_eq_token: BangEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 251,
                                                                            end: 253,
                                                                            as_str(): "!=",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 254,
                                                                                    end: 255,
                                                                                    as_str(): "2",
                                                                                },
                                                                                parsed: 2,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U16,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 255,
                                                                                            end: 258,
                                                                                            as_str(): "u16",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 245,
                                                            end: 259,
                                                            as_str(): "(1u16 != 2u16)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 259,
                                                            end: 260,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 266,
                                                                        end: 272,
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
                                                                    lhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 273,
                                                                                    end: 274,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 274,
                                                                                            end: 277,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 278,
                                                                            end: 280,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 281,
                                                                                    end: 282,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 282,
                                                                                            end: 285,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 272,
                                                            end: 286,
                                                            as_str(): "(1u32 == 1u32)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 286,
                                                            end: 287,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 292,
                                                                        end: 298,
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
                                                                NotEqual {
                                                                    lhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 299,
                                                                                    end: 300,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 300,
                                                                                            end: 303,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    bang_eq_token: BangEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 304,
                                                                            end: 306,
                                                                            as_str(): "!=",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 307,
                                                                                    end: 308,
                                                                                    as_str(): "2",
                                                                                },
                                                                                parsed: 2,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 308,
                                                                                            end: 311,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 298,
                                                            end: 312,
                                                            as_str(): "(1u32 != 2u32)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 312,
                                                            end: 313,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 319,
                                                                        end: 325,
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
                                                                    lhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 326,
                                                                                    end: 327,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 327,
                                                                                            end: 330,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 331,
                                                                            end: 333,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 334,
                                                                                    end: 335,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 335,
                                                                                            end: 338,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 325,
                                                            end: 339,
                                                            as_str(): "(1u64 == 1u64)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 339,
                                                            end: 340,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 345,
                                                                        end: 351,
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
                                                                NotEqual {
                                                                    lhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 352,
                                                                                    end: 353,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 353,
                                                                                            end: 356,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    bang_eq_token: BangEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 357,
                                                                            end: 359,
                                                                            as_str(): "!=",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 360,
                                                                                    end: 361,
                                                                                    as_str(): "2",
                                                                                },
                                                                                parsed: 2,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 361,
                                                                                            end: 364,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 351,
                                                            end: 365,
                                                            as_str(): "(1u64 != 2u64)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 365,
                                                            end: 366,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 372,
                                                                        end: 378,
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
                                                                    lhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 379,
                                                                                    end: 383,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 384,
                                                                            end: 386,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 387,
                                                                                    end: 391,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 378,
                                                            end: 392,
                                                            as_str(): "(true == true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 392,
                                                            end: 393,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 398,
                                                                        end: 404,
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
                                                                NotEqual {
                                                                    lhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 405,
                                                                                    end: 409,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    bang_eq_token: BangEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 410,
                                                                            end: 412,
                                                                            as_str(): "!=",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 413,
                                                                                    end: 418,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 404,
                                                            end: 419,
                                                            as_str(): "(true != false)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 419,
                                                            end: 420,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 426,
                                                            end: 429,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e6588390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                ),
                                                                start: 430,
                                                                end: 434,
                                                                as_str(): "zero",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 435,
                                                            end: 436,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e6588390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                    ),
                                                                    start: 437,
                                                                    end: 503,
                                                                    as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 503,
                                                            end: 504,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 509,
                                                            end: 512,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e6588390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                ),
                                                                start: 513,
                                                                end: 516,
                                                                as_str(): "one",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 517,
                                                            end: 518,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e6588390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                    ),
                                                                    start: 519,
                                                                    end: 585,
                                                                    as_str(): "0x0000000000000000000000000000000000000000000000000000000000000001",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 585,
                                                            end: 586,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 591,
                                                                        end: 597,
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
                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                        ),
                                                                                        start: 598,
                                                                                        end: 602,
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
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 603,
                                                                            end: 605,
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
                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                        ),
                                                                                        start: 606,
                                                                                        end: 610,
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
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 597,
                                                            end: 611,
                                                            as_str(): "(zero == zero)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 611,
                                                            end: 612,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 617,
                                                                        end: 623,
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
                                                                NotEqual {
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                        ),
                                                                                        start: 624,
                                                                                        end: 628,
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
                                                                    bang_eq_token: BangEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 629,
                                                                            end: 631,
                                                                            as_str(): "!=",
                                                                        },
                                                                    },
                                                                    rhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                        ),
                                                                                        start: 632,
                                                                                        end: 635,
                                                                                        as_str(): "one",
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 623,
                                                            end: 636,
                                                            as_str(): "(zero != one)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 636,
                                                            end: 637,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 663,
                                                                        end: 669,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 670,
                                                                                            end: 680,
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
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 680,
                                                                                                end: 682,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                    ),
                                                                                                    start: 682,
                                                                                                    end: 686,
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
                                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                        ),
                                                                                                        start: 687,
                                                                                                        end: 691,
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
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 686,
                                                                                end: 692,
                                                                                as_str(): "(zero)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
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
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 696,
                                                                                            end: 706,
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
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 706,
                                                                                                end: 708,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                    ),
                                                                                                    start: 708,
                                                                                                    end: 712,
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
                                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                        ),
                                                                                                        start: 713,
                                                                                                        end: 717,
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
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 712,
                                                                                end: 718,
                                                                                as_str(): "(zero)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 669,
                                                            end: 719,
                                                            as_str(): "(ContractId::from(zero) == ContractId::from(zero))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 725,
                                                                        end: 731,
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
                                                                NotEqual {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 732,
                                                                                            end: 742,
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
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 742,
                                                                                                end: 744,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                    ),
                                                                                                    start: 744,
                                                                                                    end: 748,
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
                                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                        ),
                                                                                                        start: 749,
                                                                                                        end: 753,
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
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 748,
                                                                                end: 754,
                                                                                as_str(): "(zero)",
                                                                            },
                                                                        },
                                                                    },
                                                                    bang_eq_token: BangEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 755,
                                                                            end: 757,
                                                                            as_str(): "!=",
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
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 758,
                                                                                            end: 768,
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
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 768,
                                                                                                end: 770,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                    ),
                                                                                                    start: 770,
                                                                                                    end: 774,
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
                                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                        ),
                                                                                                        start: 775,
                                                                                                        end: 778,
                                                                                                        as_str(): "one",
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
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 774,
                                                                                end: 779,
                                                                                as_str(): "(one)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 731,
                                                            end: 780,
                                                            as_str(): "(ContractId::from(zero) != ContractId::from(one))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 780,
                                                            end: 781,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 787,
                                                                        end: 793,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 794,
                                                                                            end: 801,
                                                                                            as_str(): "Address",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 801,
                                                                                                end: 803,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                    ),
                                                                                                    start: 803,
                                                                                                    end: 807,
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
                                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                        ),
                                                                                                        start: 808,
                                                                                                        end: 812,
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
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 807,
                                                                                end: 813,
                                                                                as_str(): "(zero)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 814,
                                                                            end: 816,
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
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 817,
                                                                                            end: 824,
                                                                                            as_str(): "Address",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 824,
                                                                                                end: 826,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                    ),
                                                                                                    start: 826,
                                                                                                    end: 830,
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
                                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                        ),
                                                                                                        start: 831,
                                                                                                        end: 835,
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
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 830,
                                                                                end: 836,
                                                                                as_str(): "(zero)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 793,
                                                            end: 837,
                                                            as_str(): "(Address::from(zero) == Address::from(zero))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 837,
                                                            end: 838,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 843,
                                                                        end: 849,
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
                                                                NotEqual {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 850,
                                                                                            end: 857,
                                                                                            as_str(): "Address",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 857,
                                                                                                end: 859,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                    ),
                                                                                                    start: 859,
                                                                                                    end: 863,
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
                                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                        ),
                                                                                                        start: 864,
                                                                                                        end: 868,
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
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 863,
                                                                                end: 869,
                                                                                as_str(): "(zero)",
                                                                            },
                                                                        },
                                                                    },
                                                                    bang_eq_token: BangEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 870,
                                                                            end: 872,
                                                                            as_str(): "!=",
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
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 873,
                                                                                            end: 880,
                                                                                            as_str(): "Address",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 880,
                                                                                                end: 882,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                    ),
                                                                                                    start: 882,
                                                                                                    end: 886,
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
                                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                        ),
                                                                                                        start: 887,
                                                                                                        end: 890,
                                                                                                        as_str(): "one",
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
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 886,
                                                                                end: 891,
                                                                                as_str(): "(one)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 849,
                                                            end: 892,
                                                            as_str(): "(Address::from(zero) != Address::from(one))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 892,
                                                            end: 893,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 899,
                                                                        end: 905,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 906,
                                                                                            end: 910,
                                                                                            as_str(): "B512",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 910,
                                                                                                end: 912,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                    ),
                                                                                                    start: 912,
                                                                                                    end: 916,
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
                                                                                    Tuple(
                                                                                        Parens {
                                                                                            inner: Cons {
                                                                                                head: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 918,
                                                                                                                    end: 922,
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
                                                                                                comma_token: CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                        ),
                                                                                                        start: 922,
                                                                                                        end: 923,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                                tail: Punctuated {
                                                                                                    value_separator_pairs: [],
                                                                                                    final_value_opt: Some(
                                                                                                        Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 924,
                                                                                                                            end: 928,
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
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 917,
                                                                                                end: 929,
                                                                                                as_str(): "(zero, zero)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 916,
                                                                                end: 930,
                                                                                as_str(): "((zero, zero))",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 931,
                                                                            end: 933,
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
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 934,
                                                                                            end: 938,
                                                                                            as_str(): "B512",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 938,
                                                                                                end: 940,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                    ),
                                                                                                    start: 940,
                                                                                                    end: 944,
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
                                                                                    Tuple(
                                                                                        Parens {
                                                                                            inner: Cons {
                                                                                                head: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 946,
                                                                                                                    end: 950,
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
                                                                                                comma_token: CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                        ),
                                                                                                        start: 950,
                                                                                                        end: 951,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                                tail: Punctuated {
                                                                                                    value_separator_pairs: [],
                                                                                                    final_value_opt: Some(
                                                                                                        Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 952,
                                                                                                                            end: 956,
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
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 945,
                                                                                                end: 957,
                                                                                                as_str(): "(zero, zero)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 944,
                                                                                end: 958,
                                                                                as_str(): "((zero, zero))",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 905,
                                                            end: 959,
                                                            as_str(): "(B512::from((zero, zero)) == B512::from((zero, zero)))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 959,
                                                            end: 960,
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
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 965,
                                                                        end: 971,
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
                                                                NotEqual {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 972,
                                                                                            end: 976,
                                                                                            as_str(): "B512",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 976,
                                                                                                end: 978,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                    ),
                                                                                                    start: 978,
                                                                                                    end: 982,
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
                                                                                    Tuple(
                                                                                        Parens {
                                                                                            inner: Cons {
                                                                                                head: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 984,
                                                                                                                    end: 988,
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
                                                                                                comma_token: CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                        ),
                                                                                                        start: 988,
                                                                                                        end: 989,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                                tail: Punctuated {
                                                                                                    value_separator_pairs: [],
                                                                                                    final_value_opt: Some(
                                                                                                        Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 990,
                                                                                                                            end: 994,
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
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 983,
                                                                                                end: 995,
                                                                                                as_str(): "(zero, zero)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 982,
                                                                                end: 996,
                                                                                as_str(): "((zero, zero))",
                                                                            },
                                                                        },
                                                                    },
                                                                    bang_eq_token: BangEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 997,
                                                                            end: 999,
                                                                            as_str(): "!=",
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
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 1000,
                                                                                            end: 1004,
                                                                                            as_str(): "B512",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 1004,
                                                                                                end: 1006,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1006,
                                                                                                    end: 1010,
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
                                                                                    Tuple(
                                                                                        Parens {
                                                                                            inner: Cons {
                                                                                                head: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1012,
                                                                                                                    end: 1016,
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
                                                                                                comma_token: CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1016,
                                                                                                        end: 1017,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                                tail: Punctuated {
                                                                                                    value_separator_pairs: [],
                                                                                                    final_value_opt: Some(
                                                                                                        Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1018,
                                                                                                                            end: 1021,
                                                                                                                            as_str(): "one",
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
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 1011,
                                                                                                end: 1022,
                                                                                                as_str(): "(zero, one)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 1010,
                                                                                end: 1023,
                                                                                as_str(): "((zero, one))",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 971,
                                                            end: 1024,
                                                            as_str(): "(B512::from((zero, zero)) != B512::from((zero, one)))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 1024,
                                                            end: 1025,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 1031,
                                                            end: 1035,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 135,
                                        end: 1037,
                                        as_str(): "{\n    // Primitive types\n    assert(1u8 == 1u8);\n    assert(1u8 != 2u8);\n\n    assert(1u16 == 1u16);\n    assert(1u16 != 2u16);\n\n    assert(1u32 == 1u32);\n    assert(1u32 != 2u32);\n\n    assert(1u64 == 1u64);\n    assert(1u64 != 2u64);\n\n    assert(true == true);\n    assert(true != false);\n\n    let zero = 0x0000000000000000000000000000000000000000000000000000000000000000;\n    let one = 0x0000000000000000000000000000000000000000000000000000000000000001;\n    assert(zero == zero);\n    assert(zero != one);\n\n    // stdlib types\n    assert(ContractId::from(zero) == ContractId::from(zero));\n    assert(ContractId::from(zero) != ContractId::from(one));\n\n    assert(Address::from(zero) == Address::from(zero));\n    assert(Address::from(zero) != Address::from(one));\n\n    assert(B512::from((zero, zero)) == B512::from((zero, zero)));\n    assert(B512::from((zero, zero)) != B512::from((zero, one)));\n\n    true\n}",
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
