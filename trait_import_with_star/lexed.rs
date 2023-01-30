Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe033497b40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe033497b40,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
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
                                        src (ptr): 0x00007fe033497b40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                        ),
                                        start: 8,
                                        end: 11,
                                        as_str(): "dep",
                                    },
                                },
                                path: DependencyPath {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe033497b40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 21,
                                            as_str(): "shiftable",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe033497b40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
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
                                        src (ptr): 0x00007fe033497b40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                        ),
                                        start: 23,
                                        end: 26,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe033497b40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                            ),
                                            start: 27,
                                            end: 36,
                                            as_str(): "shiftable",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe033497b40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                            ),
                                            start: 36,
                                            end: 38,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Glob {
                                        star_token: StarToken {
                                            span: Span {
                                                src (ptr): 0x00007fe033497b40,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                ),
                                                start: 38,
                                                end: 39,
                                                as_str(): "*",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe033497b40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
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
                                            src (ptr): 0x00007fe033497b40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                            ),
                                            start: 42,
                                            end: 44,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe033497b40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
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
                                            src (ptr): 0x00007fe033497b40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                            ),
                                            start: 49,
                                            end: 51,
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
                                                            src (ptr): 0x00007fe033497b40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                            ),
                                                            start: 58,
                                                            end: 61,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe033497b40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                                    ),
                                                                    start: 62,
                                                                    end: 65,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe033497b40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                                ),
                                                                start: 66,
                                                                end: 77,
                                                                as_str(): "shiftAnswer",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe033497b40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                                    ),
                                                                    start: 77,
                                                                    end: 78,
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
                                                                                src (ptr): 0x00007fe033497b40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe033497b40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                            ),
                                                            start: 83,
                                                            end: 84,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe033497b40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                                    ),
                                                                    start: 85,
                                                                    end: 86,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe033497b40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                            ),
                                                            start: 86,
                                                            end: 87,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe033497b40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                                        ),
                                                                        start: 93,
                                                                        end: 104,
                                                                        as_str(): "shiftAnswer",
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
                                                            src (ptr): 0x00007fe033497b40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                            ),
                                                            start: 104,
                                                            end: 105,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe033497b40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 108,
                                                                as_str(): "rsh",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe033497b40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
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
                                                            src (ptr): 0x00007fe033497b40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                            ),
                                                            start: 108,
                                                            end: 111,
                                                            as_str(): "(5)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe033497b40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                            ),
                                                            start: 111,
                                                            end: 112,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe033497b40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                        ),
                                        start: 52,
                                        end: 114,
                                        as_str(): "{\n    let mut shiftAnswer: u64 = 0;\n\n    shiftAnswer.rsh(5);\n}",
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
                            src (ptr): 0x00007fe0332e1c50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                            ),
                            start: 8,
                            end: 17,
                            as_str(): "shiftable",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0332e1c50,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                ),
                                start: 8,
                                end: 17,
                                as_str(): "shiftable",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0332e1c50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0332e1c50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                            ),
                                            start: 8,
                                            end: 17,
                                            as_str(): "shiftable",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0332e1c50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                        ),
                                        start: 17,
                                        end: 18,
                                        as_str(): ";",
                                    },
                                },
                                items: [
                                    Annotated {
                                        attribute_list: [],
                                        value: Trait(
                                            ItemTrait {
                                                visibility: Some(
                                                    PubToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0332e1c50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                            ),
                                                            start: 20,
                                                            end: 23,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                trait_token: TraitToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0332e1c50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                        ),
                                                        start: 24,
                                                        end: 29,
                                                        as_str(): "trait",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0332e1c50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                        ),
                                                        start: 30,
                                                        end: 39,
                                                        as_str(): "Shiftable",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics: None,
                                                where_clause_opt: None,
                                                super_traits: None,
                                                trait_items: Braces {
                                                    inner: [
                                                        (
                                                            Annotated {
                                                                attribute_list: [],
                                                                value: FnSignature {
                                                                    visibility: None,
                                                                    fn_token: FnToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                            ),
                                                                            start: 46,
                                                                            end: 48,
                                                                            as_str(): "fn",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                            ),
                                                                            start: 49,
                                                                            end: 52,
                                                                            as_str(): "lsh",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics: None,
                                                                    arguments: Parens {
                                                                        inner: NonStatic {
                                                                            self_token: SelfToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                    ),
                                                                                    start: 53,
                                                                                    end: 57,
                                                                                    as_str(): "self",
                                                                                },
                                                                            },
                                                                            ref_self: None,
                                                                            mutable_self: None,
                                                                            args_opt: Some(
                                                                                (
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                            ),
                                                                                            start: 57,
                                                                                            end: 58,
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
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 59,
                                                                                                            end: 64,
                                                                                                            as_str(): "other",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                                colon_token: ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0332e1c50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
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
                                                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                    ),
                                                                                                                    start: 66,
                                                                                                                    end: 70,
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
                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                            ),
                                                                            start: 52,
                                                                            end: 71,
                                                                            as_str(): "(self, other: Self)",
                                                                        },
                                                                    },
                                                                    return_type_opt: Some(
                                                                        (
                                                                            RightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                    ),
                                                                                    start: 72,
                                                                                    end: 74,
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
                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                ),
                                                                                                start: 75,
                                                                                                end: 79,
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
                                                            },
                                                            SemicolonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                    ),
                                                                    start: 79,
                                                                    end: 80,
                                                                    as_str(): ";",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            Annotated {
                                                                attribute_list: [],
                                                                value: FnSignature {
                                                                    visibility: None,
                                                                    fn_token: FnToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                            ),
                                                                            start: 85,
                                                                            end: 87,
                                                                            as_str(): "fn",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                            ),
                                                                            start: 88,
                                                                            end: 91,
                                                                            as_str(): "rsh",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics: None,
                                                                    arguments: Parens {
                                                                        inner: NonStatic {
                                                                            self_token: SelfToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                    ),
                                                                                    start: 92,
                                                                                    end: 96,
                                                                                    as_str(): "self",
                                                                                },
                                                                            },
                                                                            ref_self: None,
                                                                            mutable_self: None,
                                                                            args_opt: Some(
                                                                                (
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                            ),
                                                                                            start: 96,
                                                                                            end: 97,
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
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 98,
                                                                                                            end: 103,
                                                                                                            as_str(): "other",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                                colon_token: ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0332e1c50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                        ),
                                                                                                        start: 103,
                                                                                                        end: 104,
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
                                                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                    ),
                                                                                                                    start: 105,
                                                                                                                    end: 109,
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
                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                            ),
                                                                            start: 91,
                                                                            end: 110,
                                                                            as_str(): "(self, other: Self)",
                                                                        },
                                                                    },
                                                                    return_type_opt: Some(
                                                                        (
                                                                            RightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                    ),
                                                                                    start: 111,
                                                                                    end: 113,
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
                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                ),
                                                                                                start: 114,
                                                                                                end: 118,
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
                                                            },
                                                            SemicolonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                    ),
                                                                    start: 118,
                                                                    end: 119,
                                                                    as_str(): ";",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0332e1c50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                        ),
                                                        start: 40,
                                                        end: 121,
                                                        as_str(): "{\n    fn lsh(self, other: Self) -> Self;\n    fn rsh(self, other: Self) -> Self;\n}",
                                                    },
                                                },
                                                trait_defs_opt: None,
                                            },
                                        ),
                                    },
                                    Annotated {
                                        attribute_list: [],
                                        value: Impl(
                                            ItemImpl {
                                                impl_token: ImplToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0332e1c50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                        ),
                                                        start: 123,
                                                        end: 127,
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
                                                                        src (ptr): 0x00007fe0332e1c50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                        ),
                                                                        start: 128,
                                                                        end: 137,
                                                                        as_str(): "Shiftable",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                        },
                                                        ForToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0332e1c50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                ),
                                                                start: 138,
                                                                end: 141,
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
                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                    ),
                                                                    start: 142,
                                                                    end: 145,
                                                                    as_str(): "u64",
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
                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                            ),
                                                                            start: 152,
                                                                            end: 154,
                                                                            as_str(): "fn",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                            ),
                                                                            start: 155,
                                                                            end: 158,
                                                                            as_str(): "lsh",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics: None,
                                                                    arguments: Parens {
                                                                        inner: NonStatic {
                                                                            self_token: SelfToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                    ),
                                                                                    start: 159,
                                                                                    end: 163,
                                                                                    as_str(): "self",
                                                                                },
                                                                            },
                                                                            ref_self: None,
                                                                            mutable_self: None,
                                                                            args_opt: Some(
                                                                                (
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                            ),
                                                                                            start: 163,
                                                                                            end: 164,
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
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 165,
                                                                                                            end: 170,
                                                                                                            as_str(): "other",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                                colon_token: ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0332e1c50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                        ),
                                                                                                        start: 170,
                                                                                                        end: 171,
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
                                                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                    ),
                                                                                                                    start: 172,
                                                                                                                    end: 175,
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
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                            ),
                                                                            start: 158,
                                                                            end: 176,
                                                                            as_str(): "(self, other: u64)",
                                                                        },
                                                                    },
                                                                    return_type_opt: Some(
                                                                        (
                                                                            RightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                    ),
                                                                                    start: 177,
                                                                                    end: 179,
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
                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                ),
                                                                                                start: 180,
                                                                                                end: 184,
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
                                                                            Asm(
                                                                                AsmBlock {
                                                                                    asm_token: AsmToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                            ),
                                                                                            start: 195,
                                                                                            end: 198,
                                                                                            as_str(): "asm",
                                                                                        },
                                                                                    },
                                                                                    registers: Parens {
                                                                                        inner: Punctuated {
                                                                                            value_separator_pairs: [
                                                                                                (
                                                                                                    AsmRegisterDeclaration {
                                                                                                        register: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                ),
                                                                                                                start: 199,
                                                                                                                end: 201,
                                                                                                                as_str(): "r1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        value_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0332e1c50,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                        ),
                                                                                                                        start: 201,
                                                                                                                        end: 202,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Path(
                                                                                                                    PathExpr {
                                                                                                                        root_opt: None,
                                                                                                                        prefix: PathExprSegment {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                                    ),
                                                                                                                                    start: 203,
                                                                                                                                    end: 207,
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
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 207,
                                                                                                            end: 208,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    AsmRegisterDeclaration {
                                                                                                        register: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                ),
                                                                                                                start: 209,
                                                                                                                end: 211,
                                                                                                                as_str(): "r2",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        value_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0332e1c50,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                        ),
                                                                                                                        start: 211,
                                                                                                                        end: 212,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Path(
                                                                                                                    PathExpr {
                                                                                                                        root_opt: None,
                                                                                                                        prefix: PathExprSegment {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                                    ),
                                                                                                                                    start: 213,
                                                                                                                                    end: 218,
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
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 218,
                                                                                                            end: 219,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            final_value_opt: Some(
                                                                                                AsmRegisterDeclaration {
                                                                                                    register: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 220,
                                                                                                            end: 222,
                                                                                                            as_str(): "r3",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    value_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                            ),
                                                                                            start: 198,
                                                                                            end: 223,
                                                                                            as_str(): "(r1: self, r2: other, r3)",
                                                                                        },
                                                                                    },
                                                                                    contents: Braces {
                                                                                        inner: AsmBlockContents {
                                                                                            instructions: [
                                                                                                (
                                                                                                    Sll {
                                                                                                        token: SllOpcode {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                ),
                                                                                                                start: 238,
                                                                                                                end: 241,
                                                                                                                as_str(): "sll",
                                                                                                            },
                                                                                                        },
                                                                                                        ret: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                ),
                                                                                                                start: 242,
                                                                                                                end: 244,
                                                                                                                as_str(): "r3",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        lhs: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                ),
                                                                                                                start: 245,
                                                                                                                end: 247,
                                                                                                                as_str(): "r1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        rhs: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                ),
                                                                                                                start: 248,
                                                                                                                end: 250,
                                                                                                                as_str(): "r2",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                    SemicolonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 250,
                                                                                                            end: 251,
                                                                                                            as_str(): ";",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            final_expr_opt: Some(
                                                                                                AsmFinalExpr {
                                                                                                    register: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 264,
                                                                                                            end: 266,
                                                                                                            as_str(): "r3",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    ty_opt: Some(
                                                                                                        (
                                                                                                            ColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                    ),
                                                                                                                    start: 266,
                                                                                                                    end: 267,
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
                                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                                ),
                                                                                                                                start: 268,
                                                                                                                                end: 271,
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
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                            ),
                                                                                            start: 224,
                                                                                            end: 281,
                                                                                            as_str(): "{\n            sll r3 r1 r2;\n            r3: u64\n        }",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0332e1c50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                        ),
                                                                        start: 185,
                                                                        end: 287,
                                                                        as_str(): "{\n        asm(r1: self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        Annotated {
                                                            attribute_list: [],
                                                            value: ItemFn {
                                                                fn_signature: FnSignature {
                                                                    visibility: None,
                                                                    fn_token: FnToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                            ),
                                                                            start: 292,
                                                                            end: 294,
                                                                            as_str(): "fn",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                            ),
                                                                            start: 295,
                                                                            end: 298,
                                                                            as_str(): "rsh",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics: None,
                                                                    arguments: Parens {
                                                                        inner: NonStatic {
                                                                            self_token: SelfToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                    ),
                                                                                    start: 299,
                                                                                    end: 303,
                                                                                    as_str(): "self",
                                                                                },
                                                                            },
                                                                            ref_self: None,
                                                                            mutable_self: None,
                                                                            args_opt: Some(
                                                                                (
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                            ),
                                                                                            start: 303,
                                                                                            end: 304,
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
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 305,
                                                                                                            end: 310,
                                                                                                            as_str(): "other",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                                colon_token: ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0332e1c50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                        ),
                                                                                                        start: 310,
                                                                                                        end: 311,
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
                                                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                    ),
                                                                                                                    start: 312,
                                                                                                                    end: 315,
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
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                            ),
                                                                            start: 298,
                                                                            end: 316,
                                                                            as_str(): "(self, other: u64)",
                                                                        },
                                                                    },
                                                                    return_type_opt: Some(
                                                                        (
                                                                            RightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                    ),
                                                                                    start: 317,
                                                                                    end: 319,
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
                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                ),
                                                                                                start: 320,
                                                                                                end: 324,
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
                                                                            Asm(
                                                                                AsmBlock {
                                                                                    asm_token: AsmToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                            ),
                                                                                            start: 335,
                                                                                            end: 338,
                                                                                            as_str(): "asm",
                                                                                        },
                                                                                    },
                                                                                    registers: Parens {
                                                                                        inner: Punctuated {
                                                                                            value_separator_pairs: [
                                                                                                (
                                                                                                    AsmRegisterDeclaration {
                                                                                                        register: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                ),
                                                                                                                start: 339,
                                                                                                                end: 341,
                                                                                                                as_str(): "r1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        value_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0332e1c50,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                        ),
                                                                                                                        start: 341,
                                                                                                                        end: 342,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Path(
                                                                                                                    PathExpr {
                                                                                                                        root_opt: None,
                                                                                                                        prefix: PathExprSegment {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                                    ),
                                                                                                                                    start: 343,
                                                                                                                                    end: 347,
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
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 347,
                                                                                                            end: 348,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    AsmRegisterDeclaration {
                                                                                                        register: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                ),
                                                                                                                start: 349,
                                                                                                                end: 351,
                                                                                                                as_str(): "r2",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        value_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0332e1c50,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                        ),
                                                                                                                        start: 351,
                                                                                                                        end: 352,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Path(
                                                                                                                    PathExpr {
                                                                                                                        root_opt: None,
                                                                                                                        prefix: PathExprSegment {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                                    ),
                                                                                                                                    start: 353,
                                                                                                                                    end: 358,
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
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 358,
                                                                                                            end: 359,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            final_value_opt: Some(
                                                                                                AsmRegisterDeclaration {
                                                                                                    register: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 360,
                                                                                                            end: 362,
                                                                                                            as_str(): "r3",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    value_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                            ),
                                                                                            start: 338,
                                                                                            end: 363,
                                                                                            as_str(): "(r1: self, r2: other, r3)",
                                                                                        },
                                                                                    },
                                                                                    contents: Braces {
                                                                                        inner: AsmBlockContents {
                                                                                            instructions: [
                                                                                                (
                                                                                                    Srl {
                                                                                                        token: SrlOpcode {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                ),
                                                                                                                start: 378,
                                                                                                                end: 381,
                                                                                                                as_str(): "srl",
                                                                                                            },
                                                                                                        },
                                                                                                        ret: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                ),
                                                                                                                start: 382,
                                                                                                                end: 384,
                                                                                                                as_str(): "r3",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        lhs: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                ),
                                                                                                                start: 385,
                                                                                                                end: 387,
                                                                                                                as_str(): "r1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        rhs: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                ),
                                                                                                                start: 388,
                                                                                                                end: 390,
                                                                                                                as_str(): "r2",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                    SemicolonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 390,
                                                                                                            end: 391,
                                                                                                            as_str(): ";",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            final_expr_opt: Some(
                                                                                                AsmFinalExpr {
                                                                                                    register: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                            ),
                                                                                                            start: 404,
                                                                                                            end: 406,
                                                                                                            as_str(): "r3",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    ty_opt: Some(
                                                                                                        (
                                                                                                            ColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0332e1c50,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                    ),
                                                                                                                    start: 406,
                                                                                                                    end: 407,
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
                                                                                                                                src (ptr): 0x00007fe0332e1c50,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                                                                ),
                                                                                                                                start: 408,
                                                                                                                                end: 411,
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
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0332e1c50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                                            ),
                                                                                            start: 364,
                                                                                            end: 421,
                                                                                            as_str(): "{\n            srl r3 r1 r2;\n            r3: u64\n        }",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0332e1c50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                                        ),
                                                                        start: 325,
                                                                        end: 427,
                                                                        as_str(): "{\n        asm(r1: self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0332e1c50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                                        ),
                                                        start: 146,
                                                        end: 429,
                                                        as_str(): "{\n    fn lsh(self, other: u64) -> Self {\n        asm(r1: self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }\n    fn rsh(self, other: u64) -> Self {\n        asm(r1: self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }\n}",
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
