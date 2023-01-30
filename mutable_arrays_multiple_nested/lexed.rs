Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe096a57c60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe096a57c60,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
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
                                            src (ptr): 0x00007fe096a57c60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe096a57c60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
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
                                            src (ptr): 0x00007fe096a57c60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
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
                                                    src (ptr): 0x00007fe096a57c60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                    ),
                                                    start: 19,
                                                    end: 21,
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
                                                                src (ptr): 0x00007fe096a57c60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                ),
                                                                start: 22,
                                                                end: 25,
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
                                                            src (ptr): 0x00007fe096a57c60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                            ),
                                                            start: 32,
                                                            end: 35,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096a57c60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                    ),
                                                                    start: 36,
                                                                    end: 39,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe096a57c60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                ),
                                                                start: 40,
                                                                end: 41,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe096a57c60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                            ),
                                                            start: 42,
                                                            end: 43,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Array(
                                                                            SquareBrackets {
                                                                                inner: Sequence(
                                                                                    Punctuated {
                                                                                        value_separator_pairs: [
                                                                                            (
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe096a57c60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                                                                ),
                                                                                                                start: 46,
                                                                                                                end: 47,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                            parsed: 0,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe096a57c60,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                                                        ),
                                                                                                        start: 47,
                                                                                                        end: 48,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        final_value_opt: Some(
                                                                                            Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe096a57c60,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                                                            ),
                                                                                                            start: 48,
                                                                                                            end: 49,
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
                                                                                    src (ptr): 0x00007fe096a57c60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 45,
                                                                                    end: 50,
                                                                                    as_str(): "[0,1]",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe096a57c60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                ),
                                                                start: 44,
                                                                end: 51,
                                                                as_str(): "[[0,1]]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe096a57c60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                            ),
                                                            start: 51,
                                                            end: 52,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: Index {
                                                        target: Index {
                                                            target: Var(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe096a57c60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                        ),
                                                                        start: 57,
                                                                        end: 58,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            arg: SquareBrackets {
                                                                inner: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe096a57c60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                                ),
                                                                                start: 59,
                                                                                end: 60,
                                                                                as_str(): "0",
                                                                            },
                                                                            parsed: 0,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096a57c60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                    ),
                                                                    start: 58,
                                                                    end: 61,
                                                                    as_str(): "[0]",
                                                                },
                                                            },
                                                        },
                                                        arg: SquareBrackets {
                                                            inner: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe096a57c60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                            ),
                                                                            start: 62,
                                                                            end: 63,
                                                                            as_str(): "0",
                                                                        },
                                                                        parsed: 0,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe096a57c60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                ),
                                                                start: 61,
                                                                end: 64,
                                                                as_str(): "[0]",
                                                            },
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe096a57c60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                            ),
                                                            start: 65,
                                                            end: 66,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096a57c60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                    ),
                                                                    start: 67,
                                                                    end: 68,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe096a57c60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                            ),
                                                            start: 68,
                                                            end: 69,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Index {
                                                target: Index {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe096a57c60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                        ),
                                                                        start: 74,
                                                                        end: 75,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    arg: SquareBrackets {
                                                        inner: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe096a57c60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                        ),
                                                                        start: 76,
                                                                        end: 77,
                                                                        as_str(): "0",
                                                                    },
                                                                    parsed: 0,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe096a57c60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                            ),
                                                            start: 75,
                                                            end: 78,
                                                            as_str(): "[0]",
                                                        },
                                                    },
                                                },
                                                arg: SquareBrackets {
                                                    inner: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096a57c60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                    ),
                                                                    start: 79,
                                                                    end: 80,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe096a57c60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                        ),
                                                        start: 78,
                                                        end: 81,
                                                        as_str(): "[0]",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe096a57c60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 83,
                                        as_str(): "{\n    let mut a = [[0,1]];\n    a[0][0] = 1;\n    a[0][0]\n}",
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
