Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb13dd43f20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb13dd43f20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
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
                                            src (ptr): 0x00007fb13dd43f20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13dd43f20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
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
                                            src (ptr): 0x00007fb13dd43f20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
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
                                                                src (ptr): 0x00007fb13dd43f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                ),
                                                                start: 25,
                                                                end: 28,
                                                                as_str(): "asm",
                                                            },
                                                        },
                                                        registers: Parens {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb13dd43f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                ),
                                                                start: 28,
                                                                end: 30,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                        contents: Braces {
                                                            inner: AsmBlockContents {
                                                                instructions: [],
                                                                final_expr_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb13dd43f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                ),
                                                                start: 31,
                                                                end: 38,
                                                                as_str(): "{\n    }",
                                                            },
                                                        },
                                                    },
                                                ),
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13dd43f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                            ),
                                                            start: 38,
                                                            end: 39,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Asm(
                                                    AsmBlock {
                                                        asm_token: AsmToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13dd43f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                ),
                                                                start: 45,
                                                                end: 48,
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
                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                    ),
                                                                                    start: 49,
                                                                                    end: 51,
                                                                                    as_str(): "r1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb13dd43f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                            ),
                                                                                            start: 51,
                                                                                            end: 52,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                                    ),
                                                                                                    start: 53,
                                                                                                    end: 54,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13dd43f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                ),
                                                                                start: 54,
                                                                                end: 55,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        AsmRegisterDeclaration {
                                                                            register: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                    ),
                                                                                    start: 56,
                                                                                    end: 58,
                                                                                    as_str(): "r2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb13dd43f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                            ),
                                                                                            start: 58,
                                                                                            end: 59,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                                    ),
                                                                                                    start: 60,
                                                                                                    end: 61,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13dd43f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                ),
                                                                                start: 61,
                                                                                end: 62,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        AsmRegisterDeclaration {
                                                                            register: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                    ),
                                                                                    start: 63,
                                                                                    end: 65,
                                                                                    as_str(): "r3",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value_opt: None,
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13dd43f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                ),
                                                                                start: 65,
                                                                                end: 66,
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
                                                                                src (ptr): 0x00007fb13dd43f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                ),
                                                                                start: 67,
                                                                                end: 69,
                                                                                as_str(): "r4",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        value_opt: None,
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb13dd43f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                ),
                                                                start: 48,
                                                                end: 70,
                                                                as_str(): "(r1: 5, r2: 5, r3, r4)",
                                                            },
                                                        },
                                                        contents: Braces {
                                                            inner: AsmBlockContents {
                                                                instructions: [
                                                                    (
                                                                        Add {
                                                                            token: AddOpcode {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                    ),
                                                                                    start: 81,
                                                                                    end: 84,
                                                                                    as_str(): "add",
                                                                                },
                                                                            },
                                                                            ret: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                    ),
                                                                                    start: 85,
                                                                                    end: 87,
                                                                                    as_str(): "r3",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            lhs: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                    ),
                                                                                    start: 88,
                                                                                    end: 90,
                                                                                    as_str(): "r1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            rhs: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                    ),
                                                                                    start: 91,
                                                                                    end: 93,
                                                                                    as_str(): "r2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13dd43f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                ),
                                                                                start: 93,
                                                                                end: 94,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        Add {
                                                                            token: AddOpcode {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                    ),
                                                                                    start: 103,
                                                                                    end: 106,
                                                                                    as_str(): "add",
                                                                                },
                                                                            },
                                                                            ret: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                    ),
                                                                                    start: 107,
                                                                                    end: 109,
                                                                                    as_str(): "r4",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            lhs: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                    ),
                                                                                    start: 110,
                                                                                    end: 112,
                                                                                    as_str(): "r2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            rhs: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13dd43f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                    ),
                                                                                    start: 113,
                                                                                    end: 115,
                                                                                    as_str(): "r2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13dd43f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                                ),
                                                                                start: 115,
                                                                                end: 116,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_expr_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb13dd43f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                                ),
                                                                start: 71,
                                                                end: 122,
                                                                as_str(): "{\n        add r3 r1 r2;\n        add r4 r2 r2;\n    }",
                                                            },
                                                        },
                                                    },
                                                ),
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13dd43f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                            ),
                                                            start: 122,
                                                            end: 123,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb13dd43f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                        ),
                                        start: 19,
                                        end: 125,
                                        as_str(): "{\n    asm() {\n    };\n\n    asm(r1: 5, r2: 5, r3, r4) {\n        add r3 r1 r2;\n        add r4 r2 r2;\n    };\n}",
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
