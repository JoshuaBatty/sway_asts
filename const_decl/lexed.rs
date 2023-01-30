Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb12d4a1aa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb12d4a1aa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
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
                                        src (ptr): 0x00007fb12d4a1aa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 14,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb12d4a1aa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                        ),
                                        start: 15,
                                        end: 25,
                                        as_str(): "GLOBAL_VAL",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb12d4a1aa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                ),
                                                start: 25,
                                                end: 26,
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
                                                            src (ptr): 0x00007fb12d4a1aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                            ),
                                                            start: 27,
                                                            end: 30,
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
                                        src (ptr): 0x00007fb12d4a1aa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                        ),
                                        start: 31,
                                        end: 32,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fb12d4a1aa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                ),
                                                start: 33,
                                                end: 35,
                                                as_str(): "99",
                                            },
                                            parsed: 99,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb12d4a1aa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
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
                                            src (ptr): 0x00007fb12d4a1aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                            ),
                                            start: 38,
                                            end: 40,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb12d4a1aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
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
                                            src (ptr): 0x00007fb12d4a1aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
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
                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
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
                                                                src (ptr): 0x00007fb12d4a1aa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
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
                                            Item(
                                                Annotated {
                                                    attribute_list: [],
                                                    value: Const(
                                                        ItemConst {
                                                            visibility: None,
                                                            const_token: ConstToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                                    ),
                                                                    start: 61,
                                                                    end: 66,
                                                                    as_str(): "const",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                                    ),
                                                                    start: 67,
                                                                    end: 76,
                                                                    as_str(): "LOCAL_VAL",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            ty_opt: None,
                                                            eq_token: EqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                                    ),
                                                                    start: 77,
                                                                    end: 78,
                                                                    as_str(): "=",
                                                                },
                                                            },
                                                            expr: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12d4a1aa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                                            ),
                                                                            start: 79,
                                                                            end: 80,
                                                                            as_str(): "1",
                                                                        },
                                                                        parsed: 1,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            semicolon_token: SemicolonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                                    ),
                                                                    start: 80,
                                                                    end: 81,
                                                                    as_str(): ";",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Add {
                                                lhs: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                                    ),
                                                                    start: 86,
                                                                    end: 96,
                                                                    as_str(): "GLOBAL_VAL",
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
                                                        src (ptr): 0x00007fb12d4a1aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                        ),
                                                        start: 97,
                                                        end: 98,
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
                                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                                    ),
                                                                    start: 99,
                                                                    end: 108,
                                                                    as_str(): "LOCAL_VAL",
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
                                        src (ptr): 0x00007fb12d4a1aa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                        ),
                                        start: 55,
                                        end: 110,
                                        as_str(): "{\n    const LOCAL_VAL = 1;\n    GLOBAL_VAL + LOCAL_VAL\n}",
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
