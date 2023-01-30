TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: Literal(
                                    B256(
                                        [
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                        ],
                                    ),
                                ),
                                return_type: TypeId(
                                    59,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c010a00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                    ),
                                    start: 43,
                                    end: 109,
                                    as_str(): "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                59,
                            ),
                            type_ascription: TypeId(
                                7252,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14c010a00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                    ),
                    start: 32,
                    end: 110,
                    as_str(): "let addr = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c010a00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                    ),
                                                    start: 120,
                                                    end: 122,
                                                    as_str(): "==",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c010a00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                    ),
                                                    start: 120,
                                                    end: 122,
                                                    as_str(): "==",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "eq",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb14c010a00,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                ),
                                                start: 120,
                                                end: 122,
                                                as_str(): "==",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: true,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d9721f0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3022,
                                                    end: 3026,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
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
                                                    src (ptr): 0x00007fb14c010a00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                    ),
                                                    start: 118,
                                                    end: 119,
                                                    as_str(): "0",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d9721f0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3028,
                                                    end: 3033,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        1,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c010a00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                    ),
                                                    start: 123,
                                                    end: 124,
                                                    as_str(): "1",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        544,
                                        Span {
                                            src (ptr): 0x00007fb13d9721f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 3016,
                                            end: 3082,
                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
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
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c010a00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                    ),
                                    start: 118,
                                    end: 124,
                                    as_str(): "0 == 1",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
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
                                                            src (ptr): 0x00007fb14c010a00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 136,
                                                            as_str(): "0",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c010a00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                    ),
                                                    start: 135,
                                                    end: 136,
                                                    as_str(): "0",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
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
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    1,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c010a00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                                ),
                                                                start: 158,
                                                                end: 159,
                                                                as_str(): "1",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c010a00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                        ),
                                                        start: 158,
                                                        end: 159,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
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
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14c010a00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                            ),
                            start: 115,
                            end: 165,
                            as_str(): "if 0 == 1 {\n        0\n    } else {\n        1\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14c010a00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                    ),
                    start: 115,
                    end: 165,
                    as_str(): "if 0 == 1 {\n        0\n    } else {\n        1\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb14c010a00,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
        ),
        start: 9,
        end: 167,
        as_str(): "fn main() -> u64 {\n    let addr = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    if 0 == 1 {\n        0\n    } else {\n        1\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb14c010a00,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
        ),
        start: 22,
        end: 25,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

