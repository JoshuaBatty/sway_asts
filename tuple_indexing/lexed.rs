Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007f8a20305dd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007f8a20305dd0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
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
                                            src (ptr): 0x00007f8a20305dd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a20305dd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 24,
                                            as_str(): "gimme_a_pair",
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
                                            src (ptr): 0x00007f8a20305dd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                            ),
                                            start: 24,
                                            end: 26,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a20305dd0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                    ),
                                                    start: 27,
                                                    end: 29,
                                                    as_str(): "->",
                                                },
                                            },
                                            Tuple(
                                                Parens {
                                                    inner: Cons {
                                                        head: Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20305dd0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
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
                                                        comma_token: CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20305dd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                ),
                                                                start: 34,
                                                                end: 35,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                        tail: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathType {
                                                                        root_opt: None,
                                                                        prefix: PathTypeSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20305dd0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                                    ),
                                                                                    start: 36,
                                                                                    end: 39,
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
                                                        },
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20305dd0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                        ),
                                                        start: 30,
                                                        end: 40,
                                                        as_str(): "(u32, u64)",
                                                    },
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
                                            Tuple(
                                                Parens {
                                                    inner: Cons {
                                                        head: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20305dd0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                        ),
                                                                        start: 48,
                                                                        end: 49,
                                                                        as_str(): "1",
                                                                    },
                                                                    parsed: 1,
                                                                    ty_opt: Some(
                                                                        (
                                                                            U32,
                                                                            Span {
                                                                                src (ptr): 0x00007f8a20305dd0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                                ),
                                                                                start: 49,
                                                                                end: 52,
                                                                                as_str(): "u32",
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        comma_token: CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20305dd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                ),
                                                                start: 52,
                                                                end: 53,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                        tail: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20305dd0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                                ),
                                                                                start: 54,
                                                                                end: 55,
                                                                                as_str(): "2",
                                                                            },
                                                                            parsed: 2,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U64,
                                                                                    Span {
                                                                                        src (ptr): 0x00007f8a20305dd0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                                        ),
                                                                                        start: 55,
                                                                                        end: 58,
                                                                                        as_str(): "u64",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20305dd0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                        ),
                                                        start: 47,
                                                        end: 59,
                                                        as_str(): "(1u32, 2u64)",
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a20305dd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                        ),
                                        start: 41,
                                        end: 61,
                                        as_str(): "{\n    (1u32, 2u64)\n}",
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
                                            src (ptr): 0x00007f8a20305dd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                            ),
                                            start: 63,
                                            end: 65,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a20305dd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 70,
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
                                            src (ptr): 0x00007f8a20305dd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                            ),
                                            start: 70,
                                            end: 72,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a20305dd0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                    ),
                                                    start: 73,
                                                    end: 75,
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
                                                                src (ptr): 0x00007f8a20305dd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                ),
                                                                start: 76,
                                                                end: 79,
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
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20305dd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                            ),
                                                            start: 86,
                                                            end: 89,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20305dd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                ),
                                                                start: 90,
                                                                end: 91,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20305dd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                            ),
                                                            start: 92,
                                                            end: 93,
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
                                                                            src (ptr): 0x00007f8a20305dd0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                            ),
                                                                            start: 94,
                                                                            end: 106,
                                                                            as_str(): "gimme_a_pair",
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
                                                                src (ptr): 0x00007f8a20305dd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                ),
                                                                start: 106,
                                                                end: 108,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20305dd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                            ),
                                                            start: 108,
                                                            end: 109,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            TupleFieldProjection {
                                                target: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a20305dd0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                    ),
                                                                    start: 114,
                                                                    end: 115,
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
                                                        src (ptr): 0x00007f8a20305dd0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                        ),
                                                        start: 115,
                                                        end: 116,
                                                        as_str(): ".",
                                                    },
                                                },
                                                field: 0,
                                                field_span: Span {
                                                    src (ptr): 0x00007f8a20305dd0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                    ),
                                                    start: 116,
                                                    end: 117,
                                                    as_str(): "0",
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a20305dd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                        ),
                                        start: 80,
                                        end: 119,
                                        as_str(): "{\n    let x = gimme_a_pair();\n    x.0\n}",
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
