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
                            src (ptr): 0x00007fe096a57c60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
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
                                                    src (ptr): 0x00007fe096a57c60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                    ),
                                                    start: 40,
                                                    end: 41,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Array(
                                                                    ArrayExpression {
                                                                        contents: [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        0,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe096a57c60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 46,
                                                                                    end: 47,
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
                                                                                    src (ptr): 0x00007fe096a57c60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 48,
                                                                                    end: 49,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                        ],
                                                                        length_span: None,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096a57c60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                    ),
                                                                    start: 45,
                                                                    end: 50,
                                                                    as_str(): "[0,1]",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe096a57c60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                    ),
                                                    start: 44,
                                                    end: 51,
                                                    as_str(): "[[0,1]]",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe096a57c60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                    ),
                                    start: 32,
                                    end: 52,
                                    as_str(): "let mut a = [[0,1]];",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Reassignment(
                                            ReassignmentExpression {
                                                lhs: VariableExpression(
                                                    Expression {
                                                        kind: ArrayIndex(
                                                            ArrayIndexExpression {
                                                                prefix: Expression {
                                                                    kind: ArrayIndex(
                                                                        ArrayIndexExpression {
                                                                            prefix: Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe096a57c60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                                            ),
                                                                                            start: 57,
                                                                                            end: 58,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe096a57c60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 57,
                                                                                    end: 58,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                            index: Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        0,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe096a57c60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 59,
                                                                                    end: 60,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe096a57c60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                        ),
                                                                        start: 57,
                                                                        end: 61,
                                                                        as_str(): "a[0]",
                                                                    },
                                                                },
                                                                index: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe096a57c60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                        ),
                                                                        start: 62,
                                                                        end: 63,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe096a57c60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                            ),
                                                            start: 57,
                                                            end: 64,
                                                            as_str(): "a[0][0]",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            1,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe096a57c60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                        ),
                                                        start: 67,
                                                        end: 68,
                                                        as_str(): "1",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe096a57c60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                            ),
                                            start: 57,
                                            end: 68,
                                            as_str(): "a[0][0] = 1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe096a57c60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                    ),
                                    start: 57,
                                    end: 68,
                                    as_str(): "a[0][0] = 1",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: ArrayIndex(
                                            ArrayIndexExpression {
                                                prefix: Expression {
                                                    kind: ArrayIndex(
                                                        ArrayIndexExpression {
                                                            prefix: Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe096a57c60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                            ),
                                                                            start: 74,
                                                                            end: 75,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096a57c60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                    ),
                                                                    start: 74,
                                                                    end: 75,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            index: Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096a57c60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                                    ),
                                                                    start: 76,
                                                                    end: 77,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe096a57c60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                        ),
                                                        start: 74,
                                                        end: 78,
                                                        as_str(): "a[0]",
                                                    },
                                                },
                                                index: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            0,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe096a57c60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                        ),
                                                        start: 79,
                                                        end: 80,
                                                        as_str(): "0",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe096a57c60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                            ),
                                            start: 74,
                                            end: 81,
                                            as_str(): "a[0][0]",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe096a57c60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                    ),
                                    start: 74,
                                    end: 81,
                                    as_str(): "a[0][0]",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe096a57c60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                            ),
                            start: 26,
                            end: 83,
                            as_str(): "{\n    let mut a = [[0,1]];\n    a[0][0] = 1;\n    a[0][0]\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe096a57c60,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                        ),
                        start: 9,
                        end: 83,
                        as_str(): "fn main() -> u64 {\n    let mut a = [[0,1]];\n    a[0][0] = 1;\n    a[0][0]\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe096a57c60,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                        ),
                        start: 22,
                        end: 25,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe096a57c60,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
            ),
            start: 9,
            end: 83,
            as_str(): "fn main() -> u64 {\n    let mut a = [[0,1]];\n    a[0][0] = 1;\n    a[0][0]\n}",
        },
    },
]
