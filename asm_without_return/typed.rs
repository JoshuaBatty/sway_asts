TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: AsmExpression {
                            registers: [],
                            body: [],
                            returns: None,
                            whole_block_span: Span {
                                src (ptr): 0x00007fb13dd43f20,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                ),
                                start: 25,
                                end: 38,
                                as_str(): "asm() {\n    }",
                            },
                        },
                        return_type: TypeId(
                            7257,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb13dd43f20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                            ),
                            start: 25,
                            end: 38,
                            as_str(): "asm() {\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb13dd43f20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                    ),
                    start: 25,
                    end: 38,
                    as_str(): "asm() {\n    }",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: AsmExpression {
                            registers: [
                                TyAsmRegisterDeclaration {
                                    initializer: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    5,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb13dd43f20,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                ),
                                                start: 53,
                                                end: 54,
                                                as_str(): "5",
                                            },
                                        },
                                    ),
                                    name: BaseIdent {
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
                                },
                                TyAsmRegisterDeclaration {
                                    initializer: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    5,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb13dd43f20,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                                ),
                                                start: 60,
                                                end: 61,
                                                as_str(): "5",
                                            },
                                        },
                                    ),
                                    name: BaseIdent {
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
                                },
                                TyAsmRegisterDeclaration {
                                    initializer: None,
                                    name: BaseIdent {
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
                                },
                                TyAsmRegisterDeclaration {
                                    initializer: None,
                                    name: BaseIdent {
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
                                },
                            ],
                            body: [
                                AsmOp {
                                    op_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13dd43f20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                            ),
                                            start: 81,
                                            end: 84,
                                            as_str(): "add",
                                        },
                                        is_raw_ident: false,
                                    },
                                    op_args: [
                                        BaseIdent {
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
                                        BaseIdent {
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
                                        BaseIdent {
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
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb13dd43f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                        ),
                                        start: 81,
                                        end: 93,
                                        as_str(): "add r3 r1 r2",
                                    },
                                    immediate: None,
                                },
                                AsmOp {
                                    op_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13dd43f20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                            ),
                                            start: 103,
                                            end: 106,
                                            as_str(): "add",
                                        },
                                        is_raw_ident: false,
                                    },
                                    op_args: [
                                        BaseIdent {
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
                                        BaseIdent {
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
                                        BaseIdent {
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
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb13dd43f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                        ),
                                        start: 103,
                                        end: 115,
                                        as_str(): "add r4 r2 r2",
                                    },
                                    immediate: None,
                                },
                            ],
                            returns: None,
                            whole_block_span: Span {
                                src (ptr): 0x00007fb13dd43f20,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                                ),
                                start: 45,
                                end: 122,
                                as_str(): "asm(r1: 5, r2: 5, r3, r4) {\n        add r3 r1 r2;\n        add r4 r2 r2;\n    }",
                            },
                        },
                        return_type: TypeId(
                            7265,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb13dd43f20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                            ),
                            start: 45,
                            end: 122,
                            as_str(): "asm(r1: 5, r2: 5, r3, r4) {\n        add r3 r1 r2;\n        add r4 r2 r2;\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb13dd43f20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
                    ),
                    start: 45,
                    end: 122,
                    as_str(): "asm(r1: 5, r2: 5, r3, r4) {\n        add r3 r1 r2;\n        add r4 r2 r2;\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb13dd43f20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
        ),
        start: 9,
        end: 125,
        as_str(): "fn main() {\n    asm() {\n    };\n\n    asm(r1: 5, r2: 5, r3, r4) {\n        add r3 r1 r2;\n        add r4 r2 r2;\n    };\n}",
    },
    attributes: {},
    return_type: TypeId(
        7253,
    ),
    initial_return_type: TypeId(
        7252,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb13dd43f20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRzqbVZQ/asm_without_return/src/main.sw",
        ),
        start: 9,
        end: 18,
        as_str(): "fn main()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

