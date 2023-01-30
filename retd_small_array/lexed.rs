Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe06cbae200,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe06cbae200,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
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
                                            src (ptr): 0x00007fe06cbae200,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06cbae200,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 16,
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
                                            src (ptr): 0x00007fe06cbae200,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe06cbae200,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                    ),
                                                    start: 19,
                                                    end: 21,
                                                    as_str(): "->",
                                                },
                                            },
                                            Array(
                                                SquareBrackets {
                                                    inner: TyArrayDescriptor {
                                                        ty: Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06cbae200,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                                            ),
                                                                            start: 23,
                                                                            end: 26,
                                                                            as_str(): "u32",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        semicolon_token: SemicolonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cbae200,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                                ),
                                                                start: 26,
                                                                end: 27,
                                                                as_str(): ";",
                                                            },
                                                        },
                                                        length: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cbae200,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                                        ),
                                                                        start: 28,
                                                                        end: 29,
                                                                        as_str(): "1",
                                                                    },
                                                                    parsed: 1,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe06cbae200,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                        ),
                                                        start: 22,
                                                        end: 30,
                                                        as_str(): "[u32; 1]",
                                                    },
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
                                                            src (ptr): 0x00007fe06cbae200,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                            ),
                                                            start: 37,
                                                            end: 40,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cbae200,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                                ),
                                                                start: 41,
                                                                end: 42,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cbae200,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                                    ),
                                                                    start: 42,
                                                                    end: 43,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Array(
                                                                SquareBrackets {
                                                                    inner: TyArrayDescriptor {
                                                                        ty: Path(
                                                                            PathType {
                                                                                root_opt: None,
                                                                                prefix: PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06cbae200,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                                                            ),
                                                                                            start: 45,
                                                                                            end: 48,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                            },
                                                                        ),
                                                                        semicolon_token: SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06cbae200,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                                                ),
                                                                                start: 48,
                                                                                end: 49,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06cbae200,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                                                        ),
                                                                                        start: 50,
                                                                                        end: 51,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cbae200,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                                        ),
                                                                        start: 44,
                                                                        end: 52,
                                                                        as_str(): "[u32; 1]",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cbae200,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                            ),
                                                            start: 53,
                                                            end: 54,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06cbae200,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 57,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cbae200,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                                ),
                                                                start: 55,
                                                                end: 58,
                                                                as_str(): "[1]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cbae200,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                            ),
                                                            start: 58,
                                                            end: 59,
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
                                                                src (ptr): 0x00007fe06cbae200,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                                ),
                                                                start: 64,
                                                                end: 65,
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
                                        src (ptr): 0x00007fe06cbae200,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                        ),
                                        start: 31,
                                        end: 67,
                                        as_str(): "{\n    let x: [u32; 1] = [1];\n    x\n}",
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
