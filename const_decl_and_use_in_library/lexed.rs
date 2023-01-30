Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb1359d0020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb1359d0020,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
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
                                        src (ptr): 0x00007fb1359d0020,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
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
                                            src (ptr): 0x00007fb1359d0020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 19,
                                            as_str(): "consts",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1359d0020,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                        ),
                                        start: 19,
                                        end: 20,
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
                                        src (ptr): 0x00007fb1359d0020,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                        ),
                                        start: 22,
                                        end: 25,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1359d0020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                            ),
                                            start: 26,
                                            end: 32,
                                            as_str(): "consts",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb1359d0020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                            ),
                                            start: 32,
                                            end: 34,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Name {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb1359d0020,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                                ),
                                                start: 34,
                                                end: 39,
                                                as_str(): "adder",
                                            },
                                            is_raw_ident: false,
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1359d0020,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                        ),
                                        start: 39,
                                        end: 40,
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
                                            src (ptr): 0x00007fb1359d0020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                            ),
                                            start: 42,
                                            end: 44,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1359d0020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                            ),
                                            start: 45,
                                            end: 49,
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
                                            src (ptr): 0x00007fb1359d0020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                            ),
                                            start: 49,
                                            end: 51,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1359d0020,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                                    ),
                                                    start: 52,
                                                    end: 54,
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
                                                                src (ptr): 0x00007fb1359d0020,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                                                ),
                                                                start: 55,
                                                                end: 58,
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359d0020,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                                                    ),
                                                                    start: 65,
                                                                    end: 70,
                                                                    as_str(): "adder",
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
                                                        src (ptr): 0x00007fb1359d0020,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                                        ),
                                                        start: 70,
                                                        end: 72,
                                                        as_str(): "()",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1359d0020,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                        ),
                                        start: 59,
                                        end: 74,
                                        as_str(): "{\n    adder()\n}",
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
                            src (ptr): 0x00007fb12be71f40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                            ),
                            start: 8,
                            end: 14,
                            as_str(): "consts",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb12be71f40,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                ),
                                start: 8,
                                end: 14,
                                as_str(): "consts",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007fb12be71f40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb12be71f40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                            ),
                                            start: 8,
                                            end: 14,
                                            as_str(): "consts",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb12be71f40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                        ),
                                        start: 14,
                                        end: 15,
                                        as_str(): ";",
                                    },
                                },
                                items: [
                                    Annotated {
                                        attribute_list: [],
                                        value: Const(
                                            ItemConst {
                                                visibility: None,
                                                const_token: ConstToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb12be71f40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                        ),
                                                        start: 17,
                                                        end: 22,
                                                        as_str(): "const",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb12be71f40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                        ),
                                                        start: 23,
                                                        end: 28,
                                                        as_str(): "THREE",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                ty_opt: Some(
                                                    (
                                                        ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12be71f40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                                ),
                                                                start: 28,
                                                                end: 29,
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
                                                                            src (ptr): 0x00007fb12be71f40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
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
                                                    ),
                                                ),
                                                eq_token: EqToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb12be71f40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                        ),
                                                        start: 34,
                                                        end: 35,
                                                        as_str(): "=",
                                                    },
                                                },
                                                expr: Literal(
                                                    Int(
                                                        LitInt {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12be71f40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                                ),
                                                                start: 36,
                                                                end: 37,
                                                                as_str(): "3",
                                                            },
                                                            parsed: 3,
                                                            ty_opt: None,
                                                        },
                                                    ),
                                                ),
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb12be71f40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                        ),
                                                        start: 37,
                                                        end: 38,
                                                        as_str(): ";",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    Annotated {
                                        attribute_list: [],
                                        value: Const(
                                            ItemConst {
                                                visibility: None,
                                                const_token: ConstToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb12be71f40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                        ),
                                                        start: 39,
                                                        end: 44,
                                                        as_str(): "const",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb12be71f40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                        ),
                                                        start: 45,
                                                        end: 49,
                                                        as_str(): "FOUR",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                ty_opt: Some(
                                                    (
                                                        ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12be71f40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                                ),
                                                                start: 49,
                                                                end: 50,
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
                                                                            src (ptr): 0x00007fb12be71f40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
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
                                                eq_token: EqToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb12be71f40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                        ),
                                                        start: 55,
                                                        end: 56,
                                                        as_str(): "=",
                                                    },
                                                },
                                                expr: Literal(
                                                    Int(
                                                        LitInt {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12be71f40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                                ),
                                                                start: 57,
                                                                end: 58,
                                                                as_str(): "4",
                                                            },
                                                            parsed: 4,
                                                            ty_opt: None,
                                                        },
                                                    ),
                                                ),
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb12be71f40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                        ),
                                                        start: 58,
                                                        end: 59,
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
                                                    visibility: Some(
                                                        PubToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12be71f40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                                ),
                                                                start: 61,
                                                                end: 64,
                                                                as_str(): "pub",
                                                            },
                                                        },
                                                    ),
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12be71f40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                            ),
                                                            start: 65,
                                                            end: 67,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12be71f40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                            ),
                                                            start: 68,
                                                            end: 73,
                                                            as_str(): "adder",
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
                                                            src (ptr): 0x00007fb12be71f40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                            ),
                                                            start: 73,
                                                            end: 75,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12be71f40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                                    ),
                                                                    start: 76,
                                                                    end: 78,
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
                                                                                src (ptr): 0x00007fb12be71f40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                                                ),
                                                                                start: 79,
                                                                                end: 82,
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
                                                        statements: [],
                                                        final_expr_opt: Some(
                                                            Add {
                                                                lhs: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12be71f40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                                                    ),
                                                                                    start: 89,
                                                                                    end: 94,
                                                                                    as_str(): "THREE",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [],
                                                                        incomplete_suffix: false,
                                                                    },
                                                                ),
                                                                add_token: AddToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12be71f40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                                        ),
                                                                        start: 95,
                                                                        end: 96,
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
                                                                                    src (ptr): 0x00007fb12be71f40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                                                    ),
                                                                                    start: 97,
                                                                                    end: 101,
                                                                                    as_str(): "FOUR",
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
                                                        src (ptr): 0x00007fb12be71f40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                                        ),
                                                        start: 83,
                                                        end: 103,
                                                        as_str(): "{\n    THREE + FOUR\n}",
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
