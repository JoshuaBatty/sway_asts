[
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
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
                                            is_mutable: false,
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
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: MethodApplication(
                                                        MethodApplicationExpression {
                                                            method_name_binding: TypeBinding {
                                                                inner: FromTrait {
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
                                                                },
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
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
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
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            1,
                                                                        ),
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
                                                            ],
                                                        },
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
                                                then: Expression {
                                                    kind: CodeBlock(
                                                        CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: ImplicitReturnExpression(
                                                                        Expression {
                                                                            kind: Literal(
                                                                                Numeric(
                                                                                    0,
                                                                                ),
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
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fb14c010a00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                                                                ),
                                                                start: 125,
                                                                end: 142,
                                                                as_str(): "{\n        0\n    }",
                                                            },
                                                        },
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
                                                    Expression {
                                                        kind: CodeBlock(
                                                            CodeBlock {
                                                                contents: [
                                                                    AstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        1,
                                                                                    ),
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
                                                                whole_block_span: Span {
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fb14c010a00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                            ),
                            start: 26,
                            end: 167,
                            as_str(): "{\n    let addr = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    if 0 == 1 {\n        0\n    } else {\n        1\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb14c010a00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
                        ),
                        start: 9,
                        end: 167,
                        as_str(): "fn main() -> u64 {\n    let addr = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    if 0 == 1 {\n        0\n    } else {\n        1\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb14c010a00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRCHSelO/b256_bad_jumps/src/main.sw",
            ),
            start: 9,
            end: 167,
            as_str(): "fn main() -> u64 {\n    let addr = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    if 0 == 1 {\n        0\n    } else {\n        1\n    }\n}",
        },
    },
]
