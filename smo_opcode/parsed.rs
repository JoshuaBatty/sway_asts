[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
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
                    BaseIdent {
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
                ],
                import_type: Item(
                    BaseIdent {
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
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe043f82480,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
            ),
            start: 9,
            end: 39,
            as_str(): "use std::constants::ZERO_B256;",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Asm(
                                            AsmExpression {
                                                registers: [
                                                    AsmRegisterDeclaration {
                                                        name: BaseIdent {
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
                                                        initializer: Some(
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
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
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043f82480,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                    ),
                                                                    start: 80,
                                                                    end: 89,
                                                                    as_str(): "ZERO_B256",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    AsmRegisterDeclaration {
                                                        name: BaseIdent {
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
                                                        initializer: Some(
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043f82480,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                    ),
                                                                    start: 100,
                                                                    end: 101,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    AsmRegisterDeclaration {
                                                        name: BaseIdent {
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
                                                        initializer: Some(
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043f82480,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                    ),
                                                                    start: 111,
                                                                    end: 112,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    AsmRegisterDeclaration {
                                                        name: BaseIdent {
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
                                                        initializer: Some(
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043f82480,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                    ),
                                                                    start: 121,
                                                                    end: 122,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ],
                                                body: [
                                                    AsmOp {
                                                        op_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe043f82480,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                                ),
                                                                start: 134,
                                                                end: 137,
                                                                as_str(): "smo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        op_args: [
                                                            BaseIdent {
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
                                                            BaseIdent {
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
                                                            BaseIdent {
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
                                                            BaseIdent {
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
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe043f82480,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                            ),
                                                            start: 134,
                                                            end: 168,
                                                            as_str(): "smo recipient msg_len coins output",
                                                        },
                                                        immediate: None,
                                                    },
                                                ],
                                                returns: None,
                                                return_type: Tuple(
                                                    [],
                                                ),
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe043f82480,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                    ),
                                                    start: 65,
                                                    end: 175,
                                                    as_str(): "asm(recipient: ZERO_B256, msg_len: 0, output: 0, coins: 0) {\n        smo recipient msg_len coins output;\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe043f82480,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                            ),
                                            start: 65,
                                            end: 175,
                                            as_str(): "asm(recipient: ZERO_B256, msg_len: 0, output: 0, coins: 0) {\n        smo recipient msg_len coins output;\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe043f82480,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                    ),
                                    start: 65,
                                    end: 175,
                                    as_str(): "asm(recipient: ZERO_B256, msg_len: 0, output: 0, coins: 0) {\n        smo recipient msg_len coins output;\n    }",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe043f82480,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                            ),
                                            start: 180,
                                            end: 184,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe043f82480,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                    ),
                                    start: 180,
                                    end: 184,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe043f82480,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                            ),
                            start: 59,
                            end: 186,
                            as_str(): "{\n    asm(recipient: ZERO_B256, msg_len: 0, output: 0, coins: 0) {\n        smo recipient msg_len coins output;\n    }\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe043f82480,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                        ),
                        start: 41,
                        end: 186,
                        as_str(): "fn main() -> bool {\n    asm(recipient: ZERO_B256, msg_len: 0, output: 0, coins: 0) {\n        smo recipient msg_len coins output;\n    }\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe043f82480,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                        ),
                        start: 54,
                        end: 58,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe043f82480,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
            ),
            start: 41,
            end: 186,
            as_str(): "fn main() -> bool {\n    asm(recipient: ZERO_B256, msg_len: 0, output: 0, coins: 0) {\n        smo recipient msg_len coins output;\n    }\n    true\n}",
        },
    },
]
