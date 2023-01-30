Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe043f82480,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe043f82480,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
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
                                        src (ptr): 0x00007fe043f82480,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
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
                                            src (ptr): 0x00007fe043f82480,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe043f82480,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
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
                                                src (ptr): 0x00007fe043f82480,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 27,
                                                as_str(): "constants",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe043f82480,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                ),
                                                start: 27,
                                                end: 29,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe043f82480,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                    ),
                                                    start: 29,
                                                    end: 38,
                                                    as_str(): "ZERO_B256",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe043f82480,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                        ),
                                        start: 38,
                                        end: 39,
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
                                            src (ptr): 0x00007fe043f82480,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                            ),
                                            start: 41,
                                            end: 43,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe043f82480,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                            ),
                                            start: 44,
                                            end: 48,
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
                                            src (ptr): 0x00007fe043f82480,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                            ),
                                            start: 48,
                                            end: 50,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe043f82480,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                    ),
                                                    start: 51,
                                                    end: 53,
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
                                                                src (ptr): 0x00007fe043f82480,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                ),
                                                                start: 54,
                                                                end: 58,
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
                                                expr: Asm(
                                                    AsmBlock {
                                                        asm_token: AsmToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe043f82480,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                ),
                                                                start: 65,
                                                                end: 68,
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
                                                                                    src (ptr): 0x00007fe043f82480,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                    ),
                                                                                    start: 69,
                                                                                    end: 78,
                                                                                    as_str(): "recipient",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe043f82480,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                            ),
                                                                                            start: 78,
                                                                                            end: 79,
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
                                                                                                        src (ptr): 0x00007fe043f82480,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                                        ),
                                                                                                        start: 80,
                                                                                                        end: 89,
                                                                                                        as_str(): "ZERO_B256",
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
                                                                                src (ptr): 0x00007fe043f82480,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                ),
                                                                                start: 89,
                                                                                end: 90,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        AsmRegisterDeclaration {
                                                                            register: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe043f82480,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                    ),
                                                                                    start: 91,
                                                                                    end: 98,
                                                                                    as_str(): "msg_len",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe043f82480,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                            ),
                                                                                            start: 98,
                                                                                            end: 99,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe043f82480,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                                    ),
                                                                                                    start: 100,
                                                                                                    end: 101,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe043f82480,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                ),
                                                                                start: 101,
                                                                                end: 102,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        AsmRegisterDeclaration {
                                                                            register: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe043f82480,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                    ),
                                                                                    start: 103,
                                                                                    end: 109,
                                                                                    as_str(): "output",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe043f82480,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                            ),
                                                                                            start: 109,
                                                                                            end: 110,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe043f82480,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                                    ),
                                                                                                    start: 111,
                                                                                                    end: 112,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe043f82480,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                ),
                                                                                start: 112,
                                                                                end: 113,
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
                                                                                src (ptr): 0x00007fe043f82480,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                ),
                                                                                start: 114,
                                                                                end: 119,
                                                                                as_str(): "coins",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        value_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe043f82480,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                        ),
                                                                                        start: 119,
                                                                                        end: 120,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe043f82480,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                                ),
                                                                                                start: 121,
                                                                                                end: 122,
                                                                                                as_str(): "0",
                                                                                            },
                                                                                            parsed: 0,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe043f82480,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                ),
                                                                start: 68,
                                                                end: 123,
                                                                as_str(): "(recipient: ZERO_B256, msg_len: 0, output: 0, coins: 0)",
                                                            },
                                                        },
                                                        contents: Braces {
                                                            inner: AsmBlockContents {
                                                                instructions: [
                                                                    (
                                                                        Smo {
                                                                            token: SmoOpcode {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe043f82480,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                    ),
                                                                                    start: 134,
                                                                                    end: 137,
                                                                                    as_str(): "smo",
                                                                                },
                                                                            },
                                                                            addr: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe043f82480,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                    ),
                                                                                    start: 138,
                                                                                    end: 147,
                                                                                    as_str(): "recipient",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            len: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe043f82480,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                    ),
                                                                                    start: 148,
                                                                                    end: 155,
                                                                                    as_str(): "msg_len",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            output: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe043f82480,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                    ),
                                                                                    start: 156,
                                                                                    end: 161,
                                                                                    as_str(): "coins",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            coins: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe043f82480,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                    ),
                                                                                    start: 162,
                                                                                    end: 168,
                                                                                    as_str(): "output",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe043f82480,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                                ),
                                                                                start: 168,
                                                                                end: 169,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_expr_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe043f82480,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                ),
                                                                start: 124,
                                                                end: 175,
                                                                as_str(): "{\n        smo recipient msg_len coins output;\n    }",
                                                            },
                                                        },
                                                    },
                                                ),
                                                semicolon_token_opt: None,
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007fe043f82480,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                            ),
                                                            start: 180,
                                                            end: 184,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe043f82480,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                        ),
                                        start: 59,
                                        end: 186,
                                        as_str(): "{\n    asm(recipient: ZERO_B256, msg_len: 0, output: 0, coins: 0) {\n        smo recipient msg_len coins output;\n    }\n    true\n}",
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
