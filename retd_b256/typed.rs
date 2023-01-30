
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0673f9860,
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
                                        src (ptr): 0x00007fe055f5d180,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                        ),
                                        start: 145,
                                        end: 154,
                                        as_str(): "ZERO_B256",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    59,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                59,
                            ),
                            type_ascription: TypeId(
                                31631,
                            ),
                            type_ascription_span: None,
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
                                                span: Span {
                                                    src (ptr): 0x00007fe055f5d180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                    ),
                                                    start: 168,
                                                    end: 169,
                                                    as_str(): "a",
                                                },
                                                mutability: Immutable,
                                            },
                                            return_type: TypeId(
                                                59,
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
                                },
                                TyAsmRegisterDeclaration {
                                    initializer: Some(
                                        TyExpression {
                                            expression: VariableExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0673f9860,
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
                                                    src (ptr): 0x00007fe055f5d180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                    ),
                                                    start: 175,
                                                    end: 184,
                                                    as_str(): "ZERO_B256",
                                                },
                                                mutability: Immutable,
                                            },
                                            return_type: TypeId(
                                                59,
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
                        return_type: TypeId(
                            21,
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
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
                            TyExpression {
                                expression: VariableExpression {
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
                                    span: Span {
                                        src (ptr): 0x00007fe055f5d180,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                        ),
                                        start: 248,
                                        end: 249,
                                        as_str(): "a",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    59,
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
                        return_type: TypeId(
                            31637,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe055f5d180,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
        ),
        start: 113,
        end: 252,
        as_str(): "fn main() -> b256 {\n    let a = ZERO_B256;\n    asm(r1: a, r2: ZERO_B256) {\n        log r1 r2 zero zero;\n        zero\n    };\n    return a;\n}",
    },
    attributes: {},
    return_type: TypeId(
        59,
    ),
    initial_return_type: TypeId(
        59,
    ),
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

