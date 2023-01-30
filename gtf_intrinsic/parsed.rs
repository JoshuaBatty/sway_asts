[
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fb225230,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                            ),
                            start: 15,
                            end: 28,
                            as_str(): "SOME_TX_FIELD",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: Literal(
                            U64(
                                66,
                            ),
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fb225230,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                            ),
                            start: 31,
                            end: 35,
                            as_str(): "0x42",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fe0fb225230,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                        ),
                        start: 9,
                        end: 36,
                        as_str(): "const SOME_TX_FIELD = 0x42;",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fb225230,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
            ),
            start: 9,
            end: 36,
            as_str(): "const SOME_TX_FIELD = 0x42;",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fb225230,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                            ),
                            start: 43,
                            end: 62,
                            as_str(): "SOME_OTHER_TX_FIELD",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: Literal(
                            U64(
                                119,
                            ),
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fb225230,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                            ),
                            start: 65,
                            end: 69,
                            as_str(): "0x77",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fe0fb225230,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                        ),
                        start: 37,
                        end: 70,
                        as_str(): "const SOME_OTHER_TX_FIELD = 0x77;",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fb225230,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
            ),
            start: 37,
            end: 70,
            as_str(): "const SOME_OTHER_TX_FIELD = 0x77;",
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
                            src (ptr): 0x00007fe0fb225230,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                            ),
                            start: 75,
                            end: 79,
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
                                                    src (ptr): 0x00007fe0fb225230,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                    ),
                                                    start: 190,
                                                    end: 199,
                                                    as_str(): "u64_field",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: IntrinsicFunction(
                                                    IntrinsicFunctionExpression {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb225230,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                ),
                                                                start: 202,
                                                                end: 207,
                                                                as_str(): "__gtf",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        kind_binding: TypeBinding {
                                                            inner: Gtf,
                                                            type_arguments: [
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        0,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        0,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb225230,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 210,
                                                                        end: 213,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb225230,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                ),
                                                                start: 202,
                                                                end: 232,
                                                                as_str(): "__gtf::<u64>(1, SOME_TX_FIELD)",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        1,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb225230,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 215,
                                                                    end: 216,
                                                                    as_str(): "1",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb225230,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 218,
                                                                            end: 231,
                                                                            as_str(): "SOME_TX_FIELD",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb225230,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 218,
                                                                    end: 231,
                                                                    as_str(): "SOME_TX_FIELD",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb225230,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                    ),
                                                    start: 202,
                                                    end: 232,
                                                    as_str(): "__gtf::<u64>(1, SOME_TX_FIELD)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb225230,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                    ),
                                    start: 186,
                                    end: 233,
                                    as_str(): "let u64_field = __gtf::<u64>(1, SOME_TX_FIELD);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb225230,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                    ),
                                                    start: 242,
                                                    end: 252,
                                                    as_str(): "b256_field",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: IntrinsicFunction(
                                                    IntrinsicFunctionExpression {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb225230,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                ),
                                                                start: 255,
                                                                end: 260,
                                                                as_str(): "__gtf",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        kind_binding: TypeBinding {
                                                            inner: Gtf,
                                                            type_arguments: [
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        1,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        1,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb225230,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 263,
                                                                        end: 267,
                                                                        as_str(): "b256",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb225230,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                ),
                                                                start: 255,
                                                                end: 292,
                                                                as_str(): "__gtf::<b256>(2, SOME_OTHER_TX_FIELD)",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        2,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb225230,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 269,
                                                                    end: 270,
                                                                    as_str(): "2",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb225230,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 272,
                                                                            end: 291,
                                                                            as_str(): "SOME_OTHER_TX_FIELD",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb225230,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 272,
                                                                    end: 291,
                                                                    as_str(): "SOME_OTHER_TX_FIELD",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb225230,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                    ),
                                                    start: 255,
                                                    end: 292,
                                                    as_str(): "__gtf::<b256>(2, SOME_OTHER_TX_FIELD)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb225230,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                    ),
                                    start: 238,
                                    end: 293,
                                    as_str(): "let b256_field = __gtf::<b256>(2, SOME_OTHER_TX_FIELD);",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                0,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fb225230,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                            ),
                                            start: 298,
                                            end: 299,
                                            as_str(): "0",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb225230,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                    ),
                                    start: 298,
                                    end: 299,
                                    as_str(): "0",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fb225230,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                            ),
                            start: 89,
                            end: 301,
                            as_str(): "{\n    // Test expected to compile but revert because `fuel-core` does not support `gtf` yet.\n    let u64_field = __gtf::<u64>(1, SOME_TX_FIELD);\n    let b256_field = __gtf::<b256>(2, SOME_OTHER_TX_FIELD);\n    0\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0fb225230,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                        ),
                        start: 72,
                        end: 301,
                        as_str(): "fn main() -> u64 {\n    // Test expected to compile but revert because `fuel-core` does not support `gtf` yet.\n    let u64_field = __gtf::<u64>(1, SOME_TX_FIELD);\n    let b256_field = __gtf::<b256>(2, SOME_OTHER_TX_FIELD);\n    0\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fb225230,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                        ),
                        start: 85,
                        end: 88,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fb225230,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
            ),
            start: 72,
            end: 301,
            as_str(): "fn main() -> u64 {\n    // Test expected to compile but revert because `fuel-core` does not support `gtf` yet.\n    let u64_field = __gtf::<u64>(1, SOME_TX_FIELD);\n    let b256_field = __gtf::<b256>(2, SOME_OTHER_TX_FIELD);\n    0\n}",
        },
    },
]
