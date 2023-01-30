TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: Array {
                                    contents: [
                                        TyExpression {
                                            expression: Array {
                                                contents: [
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                0,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            3,
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
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            3,
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
                                            },
                                            return_type: TypeId(
                                                11,
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
                                },
                                return_type: TypeId(
                                    14,
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
                            mutability: Mutable,
                            return_type: TypeId(
                                14,
                            ),
                            type_ascription: TypeId(
                                4,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Reassignment(
                            TyReassignment {
                                lhs_base_name: BaseIdent {
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
                                lhs_type: TypeId(
                                    14,
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
                                                3,
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
                                        index_span: Span {
                                            src (ptr): 0x00007fe096a57c60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                            ),
                                            start: 59,
                                            end: 60,
                                            as_str(): "0",
                                        },
                                    },
                                    ArrayIndex {
                                        index: TyExpression {
                                            expression: Literal(
                                                U64(
                                                    0,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                3,
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
                                        index_span: Span {
                                            src (ptr): 0x00007fe096a57c60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                            ),
                                            start: 62,
                                            end: 63,
                                            as_str(): "0",
                                        },
                                    },
                                ],
                                rhs: TyExpression {
                                    expression: Literal(
                                        U64(
                                            1,
                                        ),
                                    ),
                                    return_type: TypeId(
                                        19,
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
                        return_type: TypeId(
                            21,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: ArrayIndex {
                            prefix: TyExpression {
                                expression: ArrayIndex {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
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
                                            span: Span {
                                                src (ptr): 0x00007fe096a57c60,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
                                                ),
                                                start: 74,
                                                end: 75,
                                                as_str(): "a",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            27,
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
                                    index: TyExpression {
                                        expression: Literal(
                                            U64(
                                                0,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            28,
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
                                return_type: TypeId(
                                    29,
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
                            index: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    30,
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
                        return_type: TypeId(
                            3,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe096a57c60,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR3dZ9EQ/mutable_arrays_multiple_nested/src/main.sw",
        ),
        start: 9,
        end: 83,
        as_str(): "fn main() -> u64 {\n    let mut a = [[0,1]];\n    a[0][0] = 1;\n    a[0][0]\n}",
    },
    attributes: {},
    return_type: TypeId(
        3,
    ),
    initial_return_type: TypeId(
        3,
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

