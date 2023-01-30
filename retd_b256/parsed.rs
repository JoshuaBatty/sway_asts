[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe055f5d180,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
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
                            src (ptr): 0x00007fe055f5d180,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
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
                            src (ptr): 0x00007fe055f5d180,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
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
            src (ptr): 0x00007fe055f5d180,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
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
                            src (ptr): 0x00007fe055f5d180,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                            ),
                            start: 116,
                            end: 120,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe055f5d180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                    ),
                                                    start: 141,
                                                    end: 142,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe055f5d180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                            ),
                                                            start: 145,
                                                            end: 154,
                                                            as_str(): "ZERO_B256",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe055f5d180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                    ),
                                                    start: 145,
                                                    end: 154,
                                                    as_str(): "ZERO_B256",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe055f5d180,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                    ),
                                    start: 137,
                                    end: 155,
                                    as_str(): "let a = ZERO_B256;",
                                },
                            },
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
                                                                src (ptr): 0x00007fe055f5d180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                ),
                                                                start: 164,
                                                                end: 166,
                                                                as_str(): "r1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        initializer: Some(
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe055f5d180,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                            ),
                                                                            start: 168,
                                                                            end: 169,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe055f5d180,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                    ),
                                                                    start: 168,
                                                                    end: 169,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    AsmRegisterDeclaration {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe055f5d180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                ),
                                                                start: 171,
                                                                end: 173,
                                                                as_str(): "r2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        initializer: Some(
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe055f5d180,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                            ),
                                                                            start: 175,
                                                                            end: 184,
                                                                            as_str(): "ZERO_B256",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe055f5d180,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                    ),
                                                                    start: 175,
                                                                    end: 184,
                                                                    as_str(): "ZERO_B256",
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
                                                                src (ptr): 0x00007fe055f5d180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                ),
                                                                start: 196,
                                                                end: 199,
                                                                as_str(): "log",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        op_args: [
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe055f5d180,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                    ),
                                                                    start: 200,
                                                                    end: 202,
                                                                    as_str(): "r1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe055f5d180,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                    ),
                                                                    start: 203,
                                                                    end: 205,
                                                                    as_str(): "r2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe055f5d180,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                    ),
                                                                    start: 206,
                                                                    end: 210,
                                                                    as_str(): "zero",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe055f5d180,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                    ),
                                                                    start: 211,
                                                                    end: 215,
                                                                    as_str(): "zero",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe055f5d180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                            ),
                                                            start: 196,
                                                            end: 215,
                                                            as_str(): "log r1 r2 zero zero",
                                                        },
                                                        immediate: None,
                                                    },
                                                ],
                                                returns: Some(
                                                    (
                                                        AsmRegister {
                                                            name: "zero",
                                                        },
                                                        Span {
                                                            src (ptr): 0x00007fe055f5d180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                            ),
                                                            start: 225,
                                                            end: 229,
                                                            as_str(): "zero",
                                                        },
                                                    ),
                                                ),
                                                return_type: UnsignedInteger(
                                                    SixtyFour,
                                                ),
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe055f5d180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                    ),
                                                    start: 160,
                                                    end: 235,
                                                    as_str(): "asm(r1: a, r2: ZERO_B256) {\n        log r1 r2 zero zero;\n        zero\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe055f5d180,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                            ),
                                            start: 160,
                                            end: 235,
                                            as_str(): "asm(r1: a, r2: ZERO_B256) {\n        log r1 r2 zero zero;\n        zero\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe055f5d180,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                    ),
                                    start: 160,
                                    end: 235,
                                    as_str(): "asm(r1: a, r2: ZERO_B256) {\n        log r1 r2 zero zero;\n        zero\n    }",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe055f5d180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                            ),
                                                            start: 248,
                                                            end: 249,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe055f5d180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                    ),
                                                    start: 248,
                                                    end: 249,
                                                    as_str(): "a",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe055f5d180,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                            ),
                                            start: 241,
                                            end: 249,
                                            as_str(): "return a",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe055f5d180,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                    ),
                                    start: 241,
                                    end: 249,
                                    as_str(): "return a",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe055f5d180,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                            ),
                            start: 131,
                            end: 252,
                            as_str(): "{\n    let a = ZERO_B256;\n    asm(r1: a, r2: ZERO_B256) {\n        log r1 r2 zero zero;\n        zero\n    };\n    return a;\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe055f5d180,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                        ),
                        start: 113,
                        end: 252,
                        as_str(): "fn main() -> b256 {\n    let a = ZERO_B256;\n    asm(r1: a, r2: ZERO_B256) {\n        log r1 r2 zero zero;\n        zero\n    };\n    return a;\n}",
                    },
                    return_type: B256,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe055f5d180,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                        ),
                        start: 126,
                        end: 130,
                        as_str(): "b256",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe055f5d180,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
            ),
            start: 113,
            end: 252,
            as_str(): "fn main() -> b256 {\n    let a = ZERO_B256;\n    asm(r1: a, r2: ZERO_B256) {\n        log r1 r2 zero zero;\n        zero\n    };\n    return a;\n}",
        },
    },
]
