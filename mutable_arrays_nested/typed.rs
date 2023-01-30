TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe08f988f30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
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
                                    src (ptr): 0x00007fe08f988f30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
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
                                                            src (ptr): 0x00007fe08f988f30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
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
                                                            src (ptr): 0x00007fe08f988f30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
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
                                                src (ptr): 0x00007fe08f988f30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
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
                                    src (ptr): 0x00007fe08f988f30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
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
                    src (ptr): 0x00007fe08f988f30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
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
                                        src (ptr): 0x00007fe08f988f30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
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
                                                src (ptr): 0x00007fe08f988f30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                                                ),
                                                start: 59,
                                                end: 60,
                                                as_str(): "0",
                                            },
                                        },
                                        index_span: Span {
                                            src (ptr): 0x00007fe08f988f30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                                            ),
                                            start: 59,
                                            end: 60,
                                            as_str(): "0",
                                        },
                                    },
                                ],
                                rhs: TyExpression {
                                    expression: Array {
                                        contents: [
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
                                                    src (ptr): 0x00007fe08f988f30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                                                    ),
                                                    start: 65,
                                                    end: 66,
                                                    as_str(): "1",
                                                },
                                            },
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
                                                    src (ptr): 0x00007fe08f988f30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                                                    ),
                                                    start: 68,
                                                    end: 69,
                                                    as_str(): "0",
                                                },
                                            },
                                        ],
                                    },
                                    return_type: TypeId(
                                        23,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe08f988f30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                                        ),
                                        start: 64,
                                        end: 70,
                                        as_str(): "[1, 0]",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            25,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe08f988f30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                            ),
                            start: 57,
                            end: 70,
                            as_str(): "a[0] = [1, 0]",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe08f988f30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                    ),
                    start: 57,
                    end: 70,
                    as_str(): "a[0] = [1, 0]",
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
                                                    src (ptr): 0x00007fe08f988f30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                                                    ),
                                                    start: 40,
                                                    end: 41,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe08f988f30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                                                ),
                                                start: 76,
                                                end: 77,
                                                as_str(): "a",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            31,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08f988f30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                                            ),
                                            start: 76,
                                            end: 77,
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
                                            32,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08f988f30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                                            ),
                                            start: 78,
                                            end: 79,
                                            as_str(): "0",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    33,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f988f30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                                    ),
                                    start: 76,
                                    end: 80,
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
                                    34,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f988f30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                                    ),
                                    start: 81,
                                    end: 82,
                                    as_str(): "0",
                                },
                            },
                        },
                        return_type: TypeId(
                            3,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe08f988f30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                            ),
                            start: 76,
                            end: 83,
                            as_str(): "a[0][0]",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe08f988f30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
                    ),
                    start: 76,
                    end: 83,
                    as_str(): "a[0][0]",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe08f988f30,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
        ),
        start: 9,
        end: 85,
        as_str(): "fn main() -> u64 {\n    let mut a = [[0,1]];\n    a[0] = [1, 0];\n    a[0][0]\n}",
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
        src (ptr): 0x00007fe08f988f30,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRINQNpU/mutable_arrays_nested/src/main.sw",
        ),
        start: 22,
        end: 25,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

