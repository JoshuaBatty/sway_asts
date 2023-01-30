Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb1359303f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb1359303f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
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
                                            src (ptr): 0x00007fb1359303f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1359303f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
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
                                            src (ptr): 0x00007fb1359303f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
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
                                                    src (ptr): 0x00007fb1359303f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
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
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 22,
                                                                end: 26,
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
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 44,
                                                            end: 47,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 48,
                                                                end: 49,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 50,
                                                            end: 51,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 52,
                                                                    end: 55,
                                                                    as_str(): "0xF",
                                                                },
                                                                parsed: 15,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 55,
                                                            end: 56,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 61,
                                                            end: 64,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 65,
                                                                end: 66,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 67,
                                                            end: 68,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 69,
                                                                    end: 73,
                                                                    as_str(): "0xFF",
                                                                },
                                                                parsed: 255,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 73,
                                                            end: 74,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 79,
                                                            end: 82,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 83,
                                                                end: 84,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 85,
                                                            end: 86,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 87,
                                                                    end: 92,
                                                                    as_str(): "0xFFF",
                                                                },
                                                                parsed: 4095,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 92,
                                                            end: 93,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 101,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 102,
                                                                end: 103,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 104,
                                                            end: 105,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 106,
                                                                    end: 112,
                                                                    as_str(): "0xAAAA",
                                                                },
                                                                parsed: 43690,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 112,
                                                            end: 113,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 118,
                                                            end: 121,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 122,
                                                                end: 123,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 124,
                                                            end: 125,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 126,
                                                                    end: 134,
                                                                    as_str(): "0xB_3333",
                                                                },
                                                                parsed: 734003,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 134,
                                                            end: 135,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 140,
                                                            end: 143,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 144,
                                                                end: 145,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 146,
                                                            end: 147,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 148,
                                                                    end: 159,
                                                                    as_str(): "0xFFFF_4444",
                                                                },
                                                                parsed: 4294919236,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 159,
                                                            end: 160,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 165,
                                                            end: 168,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 169,
                                                                end: 170,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 171,
                                                            end: 172,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 173,
                                                                    end: 189,
                                                                    as_str(): "0x1111_1111_1111",
                                                                },
                                                                parsed: 18764998447377,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 189,
                                                            end: 190,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 195,
                                                            end: 198,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 199,
                                                                end: 200,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 201,
                                                            end: 202,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 203,
                                                                    end: 222,
                                                                    as_str(): "0xFFFF_FFFFFFFFFFFF",
                                                                },
                                                                parsed: 18446744073709551615,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 222,
                                                            end: 223,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 228,
                                                            end: 231,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 232,
                                                                end: 233,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 234,
                                                            end: 235,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 236,
                                                                    end: 317,
                                                                    as_str(): "0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF",
                                                                },
                                                                parsed: 115792089237316195423570985008687907853269984665640564039457584007913129639935,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 317,
                                                            end: 318,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 323,
                                                            end: 326,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 327,
                                                                end: 328,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 329,
                                                            end: 330,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 331,
                                                                    end: 412,
                                                                    as_str(): "0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF",
                                                                },
                                                                parsed: 115792089237316195423570985008687907853269984665640564039457584007913129639935,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 412,
                                                            end: 413,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 433,
                                                            end: 436,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 437,
                                                                end: 438,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 439,
                                                            end: 440,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 441,
                                                                    end: 444,
                                                                    as_str(): "0b1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 444,
                                                            end: 445,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 450,
                                                            end: 453,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 454,
                                                                end: 455,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 456,
                                                            end: 457,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 458,
                                                                    end: 462,
                                                                    as_str(): "0b11",
                                                                },
                                                                parsed: 3,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 462,
                                                            end: 463,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 468,
                                                            end: 471,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 472,
                                                                end: 473,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 474,
                                                            end: 475,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 476,
                                                                    end: 481,
                                                                    as_str(): "0b111",
                                                                },
                                                                parsed: 7,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 481,
                                                            end: 482,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 487,
                                                            end: 490,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 491,
                                                                end: 492,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 493,
                                                            end: 494,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 495,
                                                                    end: 501,
                                                                    as_str(): "0b1111",
                                                                },
                                                                parsed: 15,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 501,
                                                            end: 502,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 507,
                                                            end: 510,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 511,
                                                                end: 512,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 513,
                                                            end: 514,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 515,
                                                                    end: 523,
                                                                    as_str(): "0b1_1111",
                                                                },
                                                                parsed: 31,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 523,
                                                            end: 524,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 529,
                                                            end: 532,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 533,
                                                                end: 534,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 535,
                                                            end: 536,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 537,
                                                                    end: 548,
                                                                    as_str(): "0b1111_0000",
                                                                },
                                                                parsed: 240,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 548,
                                                            end: 549,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 554,
                                                            end: 557,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 558,
                                                                end: 559,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 560,
                                                            end: 561,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 562,
                                                                    end: 578,
                                                                    as_str(): "0b1111_1111_1111",
                                                                },
                                                                parsed: 4095,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 578,
                                                            end: 579,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 584,
                                                            end: 587,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 588,
                                                                end: 589,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 590,
                                                            end: 591,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 592,
                                                                    end: 611,
                                                                    as_str(): "0b1111_111111111111",
                                                                },
                                                                parsed: 65535,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 611,
                                                            end: 612,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 617,
                                                            end: 620,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 621,
                                                                end: 622,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 623,
                                                            end: 624,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 625,
                                                                    end: 706,
                                                                    as_str(): "0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111",
                                                                },
                                                                parsed: 18446744073709551615,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 706,
                                                            end: 707,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 712,
                                                            end: 715,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359303f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                ),
                                                                start: 716,
                                                                end: 717,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 718,
                                                            end: 719,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1359303f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                                    ),
                                                                    start: 720,
                                                                    end: 1041,
                                                                    as_str(): "0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111",
                                                                },
                                                                parsed: 115792089237316195423570985008687907853269984665640564039457584007913129639935,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 1041,
                                                            end: 1042,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1359303f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                                            ),
                                                            start: 1048,
                                                            end: 1052,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1359303f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                        ),
                                        start: 27,
                                        end: 1054,
                                        as_str(): "{\n    // Hex\n    let x = 0xF;\n    let x = 0xFF;\n    let x = 0xFFF;\n    let x = 0xAAAA;\n    let x = 0xB_3333;\n    let x = 0xFFFF_4444;\n    let x = 0x1111_1111_1111;\n    let x = 0xFFFF_FFFFFFFFFFFF;\n    let x = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;\n    let x = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;\n\n    // Binary\n    let y = 0b1;\n    let y = 0b11;\n    let y = 0b111;\n    let y = 0b1111;\n    let y = 0b1_1111;\n    let y = 0b1111_0000;\n    let y = 0b1111_1111_1111;\n    let y = 0b1111_111111111111;\n    let y = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;\n    let y = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;\n\n    true\n}",
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
