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
                            src (ptr): 0x00007fe08f9c9f10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
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
                                                    src (ptr): 0x00007fe08f9c9f10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                    ),
                                                    start: 40,
                                                    end: 50,
                                                    as_str(): "my_array_0",
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
                                                        src (ptr): 0x00007fe08f9c9f10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                        ),
                                                        start: 53,
                                                        end: 56,
                                                        as_str(): "u64",
                                                    },
                                                },
                                                Length {
                                                    val: 1,
                                                    span: Span {
                                                        src (ptr): 0x00007fe08f9c9f10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                        ),
                                                        start: 58,
                                                        end: 59,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe08f9c9f10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                    ),
                                                    start: 52,
                                                    end: 60,
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
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 64,
                                                                    end: 65,
                                                                    as_str(): "1",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f9c9f10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                    ),
                                                    start: 63,
                                                    end: 66,
                                                    as_str(): "[1]",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f9c9f10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                    ),
                                    start: 32,
                                    end: 67,
                                    as_str(): "let mut my_array_0: [u64; 1] = [1];",
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
                                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                                ),
                                                                                start: 72,
                                                                                end: 82,
                                                                                as_str(): "my_array_0",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f9c9f10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                        ),
                                                                        start: 72,
                                                                        end: 82,
                                                                        as_str(): "my_array_0",
                                                                    },
                                                                },
                                                                index: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f9c9f10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                        ),
                                                                        start: 83,
                                                                        end: 84,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 72,
                                                            end: 85,
                                                            as_str(): "my_array_0[0]",
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
                                                        src (ptr): 0x00007fe08f9c9f10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                        ),
                                                        start: 88,
                                                        end: 90,
                                                        as_str(): "10",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08f9c9f10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                            ),
                                            start: 72,
                                            end: 90,
                                            as_str(): "my_array_0[0] = 10",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f9c9f10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                    ),
                                    start: 72,
                                    end: 90,
                                    as_str(): "my_array_0[0] = 10",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f9c9f10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                    ),
                                                    start: 105,
                                                    end: 115,
                                                    as_str(): "my_array_1",
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
                                                        src (ptr): 0x00007fe08f9c9f10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                        ),
                                                        start: 118,
                                                        end: 121,
                                                        as_str(): "u64",
                                                    },
                                                },
                                                Length {
                                                    val: 1,
                                                    span: Span {
                                                        src (ptr): 0x00007fe08f9c9f10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                        ),
                                                        start: 123,
                                                        end: 124,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe08f9c9f10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                    ),
                                                    start: 117,
                                                    end: 125,
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
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 129,
                                                                    end: 130,
                                                                    as_str(): "1",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f9c9f10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                    ),
                                                    start: 128,
                                                    end: 131,
                                                    as_str(): "[1]",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f9c9f10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                    ),
                                    start: 97,
                                    end: 132,
                                    as_str(): "let mut my_array_1: [u64; 1] = [1];",
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
                                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                                ),
                                                                                start: 137,
                                                                                end: 147,
                                                                                as_str(): "my_array_1",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f9c9f10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                        ),
                                                                        start: 137,
                                                                        end: 147,
                                                                        as_str(): "my_array_1",
                                                                    },
                                                                },
                                                                index: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f9c9f10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                        ),
                                                                        start: 148,
                                                                        end: 149,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 137,
                                                            end: 150,
                                                            as_str(): "my_array_1[0]",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            20,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe08f9c9f10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                        ),
                                                        start: 153,
                                                        end: 155,
                                                        as_str(): "20",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08f9c9f10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                            ),
                                            start: 137,
                                            end: 155,
                                            as_str(): "my_array_1[0] = 20",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f9c9f10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                    ),
                                    start: 137,
                                    end: 155,
                                    as_str(): "my_array_1[0] = 20",
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
                                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                                ),
                                                                                start: 162,
                                                                                end: 172,
                                                                                as_str(): "my_array_0",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f9c9f10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                        ),
                                                                        start: 162,
                                                                        end: 172,
                                                                        as_str(): "my_array_0",
                                                                    },
                                                                },
                                                                index: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f9c9f10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                        ),
                                                                        start: 173,
                                                                        end: 174,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 162,
                                                            end: 175,
                                                            as_str(): "my_array_0[0]",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: ArrayIndex(
                                                        ArrayIndexExpression {
                                                            prefix: Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08f9c9f10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                            ),
                                                                            start: 178,
                                                                            end: 188,
                                                                            as_str(): "my_array_1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 178,
                                                                    end: 188,
                                                                    as_str(): "my_array_1",
                                                                },
                                                            },
                                                            index: Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 189,
                                                                    end: 190,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe08f9c9f10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                        ),
                                                        start: 178,
                                                        end: 191,
                                                        as_str(): "my_array_1[0]",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08f9c9f10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                            ),
                                            start: 162,
                                            end: 191,
                                            as_str(): "my_array_0[0] = my_array_1[0]",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f9c9f10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                    ),
                                    start: 162,
                                    end: 191,
                                    as_str(): "my_array_0[0] = my_array_1[0]",
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
                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                ),
                                                                start: 197,
                                                                end: 207,
                                                                as_str(): "my_array_0",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe08f9c9f10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                        ),
                                                        start: 197,
                                                        end: 207,
                                                        as_str(): "my_array_0",
                                                    },
                                                },
                                                index: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            0,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe08f9c9f10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                        ),
                                                        start: 208,
                                                        end: 209,
                                                        as_str(): "0",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08f9c9f10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                            ),
                                            start: 197,
                                            end: 210,
                                            as_str(): "my_array_0[0]",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f9c9f10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                    ),
                                    start: 197,
                                    end: 210,
                                    as_str(): "my_array_0[0]",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe08f9c9f10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                            ),
                            start: 26,
                            end: 212,
                            as_str(): "{\n    let mut my_array_0: [u64; 1] = [1];\n    my_array_0[0] = 10;\n\n    let mut my_array_1: [u64; 1] = [1];\n    my_array_1[0] = 20;\n\n    my_array_0[0] = my_array_1[0];\n    my_array_0[0]\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe08f9c9f10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                        ),
                        start: 9,
                        end: 212,
                        as_str(): "fn main() -> u64 {\n    let mut my_array_0: [u64; 1] = [1];\n    my_array_0[0] = 10;\n\n    let mut my_array_1: [u64; 1] = [1];\n    my_array_1[0] = 20;\n\n    my_array_0[0] = my_array_1[0];\n    my_array_0[0]\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe08f9c9f10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                        ),
                        start: 22,
                        end: 25,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08f9c9f10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
            ),
            start: 9,
            end: 212,
            as_str(): "fn main() -> u64 {\n    let mut my_array_0: [u64; 1] = [1];\n    my_array_0[0] = 10;\n\n    let mut my_array_1: [u64; 1] = [1];\n    my_array_1[0] = 20;\n\n    my_array_0[0] = my_array_1[0];\n    my_array_0[0]\n}",
        },
    },
]
