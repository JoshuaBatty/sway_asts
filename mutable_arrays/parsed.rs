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
                            src (ptr): 0x00007fe09f253370,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
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
                                                    src (ptr): 0x00007fe09f253370,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                    ),
                                                    start: 40,
                                                    end: 48,
                                                    as_str(): "my_array",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Array(
                                                TypeArgument {
                                                    type_id: TypeId(
                                                        0,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        0,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f253370,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                        ),
                                                        start: 51,
                                                        end: 54,
                                                        as_str(): "u64",
                                                    },
                                                },
                                                Length {
                                                    val: 1,
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f253370,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                        ),
                                                        start: 56,
                                                        end: 57,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe09f253370,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                    ),
                                                    start: 50,
                                                    end: 58,
                                                    as_str(): "[u64; 1]",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        1,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f253370,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                                    ),
                                                                    start: 62,
                                                                    end: 63,
                                                                    as_str(): "1",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f253370,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                    ),
                                                    start: 61,
                                                    end: 64,
                                                    as_str(): "[1]",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f253370,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                    ),
                                    start: 32,
                                    end: 65,
                                    as_str(): "let mut my_array: [u64; 1] = [1];",
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
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f253370,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                                                ),
                                                                                start: 70,
                                                                                end: 78,
                                                                                as_str(): "my_array",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f253370,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                                        ),
                                                                        start: 70,
                                                                        end: 78,
                                                                        as_str(): "my_array",
                                                                    },
                                                                },
                                                                index: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f253370,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                                        ),
                                                                        start: 79,
                                                                        end: 80,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f253370,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                            ),
                                                            start: 70,
                                                            end: 81,
                                                            as_str(): "my_array[0]",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            10,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f253370,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                        ),
                                                        start: 84,
                                                        end: 86,
                                                        as_str(): "10",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f253370,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                            ),
                                            start: 70,
                                            end: 86,
                                            as_str(): "my_array[0] = 10",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f253370,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                    ),
                                    start: 70,
                                    end: 86,
                                    as_str(): "my_array[0] = 10",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: ArrayIndex(
                                            ArrayIndexExpression {
                                                prefix: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f253370,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                                ),
                                                                start: 92,
                                                                end: 100,
                                                                as_str(): "my_array",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f253370,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                        ),
                                                        start: 92,
                                                        end: 100,
                                                        as_str(): "my_array",
                                                    },
                                                },
                                                index: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            0,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f253370,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                                        ),
                                                        start: 101,
                                                        end: 102,
                                                        as_str(): "0",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f253370,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                            ),
                                            start: 92,
                                            end: 103,
                                            as_str(): "my_array[0]",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f253370,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                    ),
                                    start: 92,
                                    end: 103,
                                    as_str(): "my_array[0]",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09f253370,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                            ),
                            start: 26,
                            end: 105,
                            as_str(): "{\n    let mut my_array: [u64; 1] = [1];\n    my_array[0] = 10;\n    my_array[0]\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe09f253370,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                        ),
                        start: 9,
                        end: 105,
                        as_str(): "fn main() -> u64 {\n    let mut my_array: [u64; 1] = [1];\n    my_array[0] = 10;\n    my_array[0]\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09f253370,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                        ),
                        start: 22,
                        end: 25,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09f253370,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
            ),
            start: 9,
            end: 105,
            as_str(): "fn main() -> u64 {\n    let mut my_array: [u64; 1] = [1];\n    my_array[0] = 10;\n    my_array[0]\n}",
        },
    },
]
