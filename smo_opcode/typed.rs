
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: AsmExpression {
                            registers: [
                                TyAsmRegisterDeclaration {
                                    initializer: Some(
                                        TyExpression {
                                            expression: VariableExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0563d1870,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/constants.sw",
                                                        ),
                                                        start: 117,
                                                        end: 126,
                                                        as_str(): "ZERO_B256",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fe043f82480,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
                                                    ),
                                                    start: 80,
                                                    end: 89,
                                                    as_str(): "ZERO_B256",
                                                },
                                                mutability: Immutable,
                                            },
                                            return_type: TypeId(
                                                59,
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
                                },
                                TyAsmRegisterDeclaration {
                                    initializer: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    0,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
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
                                },
                                TyAsmRegisterDeclaration {
                                    initializer: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    0,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
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
                                },
                                TyAsmRegisterDeclaration {
                                    initializer: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    0,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
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
                        return_type: TypeId(
                            31641,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            Boolean(
                                true,
                            ),
                        ),
                        return_type: TypeId(
                            71,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe043f82480,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRlXcNlg/smo_opcode/src/main.sw",
        ),
        start: 41,
        end: 186,
        as_str(): "fn main() -> bool {\n    asm(recipient: ZERO_B256, msg_len: 0, output: 0, coins: 0) {\n        smo recipient msg_len coins output;\n    }\n    true\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

