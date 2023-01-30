Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb14c010a00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb14c010a00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
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
                                            src (ptr): 0x00007fb14c010a00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14c010a00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
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
                                            src (ptr): 0x00007fb14c010a00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
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
                                                    src (ptr): 0x00007fb14c010a00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
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
                                                                src (ptr): 0x00007fb14c010a00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
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
                                                            src (ptr): 0x00007fb14c010a00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                            ),
                                                            start: 32,
                                                            end: 35,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c010a00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                                ),
                                                                start: 36,
                                                                end: 40,
                                                                as_str(): "addr",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c010a00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
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
                                                                    src (ptr): 0x00007fb14c010a00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                                    ),
                                                                    start: 43,
                                                                    end: 109,
                                                                    as_str(): "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                                                                },
                                                                parsed: 77194726158210796949047323339125271902179989777093709359638389338608753093290,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c010a00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                            ),
                                                            start: 109,
                                                            end: 110,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c010a00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                            ),
                                                            start: 115,
                                                            end: 117,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        Equal {
                                                            lhs: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c010a00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                                            ),
                                                                            start: 118,
                                                                            end: 119,
                                                                            as_str(): "0",
                                                                        },
                                                                        parsed: 0,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            double_eq_token: DoubleEqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c010a00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                                    ),
                                                                    start: 120,
                                                                    end: 122,
                                                                    as_str(): "==",
                                                                },
                                                            },
                                                            rhs: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c010a00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                                            ),
                                                                            start: 123,
                                                                            end: 124,
                                                                            as_str(): "1",
                                                                        },
                                                                        parsed: 1,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c010a00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                                                ),
                                                                                start: 135,
                                                                                end: 136,
                                                                                as_str(): "0",
                                                                            },
                                                                            parsed: 0,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c010a00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                            ),
                                                            start: 125,
                                                            end: 142,
                                                            as_str(): "{\n        0\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c010a00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                                    ),
                                                                    start: 143,
                                                                    end: 147,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Break(
                                                                Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c010a00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                                                            ),
                                                                                            start: 158,
                                                                                            end: 159,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c010a00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                                        ),
                                                                        start: 148,
                                                                        end: 165,
                                                                        as_str(): "{\n        1\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14c010a00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 167,
                                        as_str(): "{\n    let addr = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    if 0 == 1 {\n        0\n    } else {\n        1\n    }\n}",
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
