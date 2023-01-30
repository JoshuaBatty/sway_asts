TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: Array {
                                    contents: [
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    1,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                0,
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
                                },
                                return_type: TypeId(
                                    9,
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
                            mutability: Mutable,
                            return_type: TypeId(
                                9,
                            ),
                            type_ascription: TypeId(
                                5,
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
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Reassignment(
                            TyReassignment {
                                lhs_base_name: BaseIdent {
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
                                lhs_type: TypeId(
                                    9,
                                ),
                                lhs_indices: [
                                    ArrayIndex {
                                        index: TyExpression {
                                            expression: Literal(
                                                U64(
                                                    0,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                0,
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
                                        index_span: Span {
                                            src (ptr): 0x00007fe09f253370,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                            ),
                                            start: 79,
                                            end: 80,
                                            as_str(): "0",
                                        },
                                    },
                                ],
                                rhs: TyExpression {
                                    expression: Literal(
                                        U64(
                                            10,
                                        ),
                                    ),
                                    return_type: TypeId(
                                        13,
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
                        return_type: TypeId(
                            15,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: ArrayIndex {
                            prefix: TyExpression {
                                expression: VariableExpression {
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
                                    span: Span {
                                        src (ptr): 0x00007fe09f253370,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
                                        ),
                                        start: 92,
                                        end: 100,
                                        as_str(): "my_array",
                                    },
                                    mutability: Mutable,
                                },
                                return_type: TypeId(
                                    18,
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
                            index: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    19,
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
                        return_type: TypeId(
                            0,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09f253370,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRl7BwLZ/mutable_arrays/src/main.sw",
        ),
        start: 9,
        end: 105,
        as_str(): "fn main() -> u64 {\n    let mut my_array: [u64; 1] = [1];\n    my_array[0] = 10;\n    my_array[0]\n}",
    },
    attributes: {},
    return_type: TypeId(
        0,
    ),
    initial_return_type: TypeId(
        0,
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

