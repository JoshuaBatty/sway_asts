TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                                },
                                return_type: TypeId(
                                    9,
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
                            mutability: Mutable,
                            return_type: TypeId(
                                9,
                            ),
                            type_ascription: TypeId(
                                5,
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
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Reassignment(
                            TyReassignment {
                                lhs_base_name: BaseIdent {
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
                                                src (ptr): 0x00007fe08f9c9f10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                ),
                                                start: 83,
                                                end: 84,
                                                as_str(): "0",
                                            },
                                        },
                                        index_span: Span {
                                            src (ptr): 0x00007fe08f9c9f10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                            ),
                                            start: 83,
                                            end: 84,
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
                        return_type: TypeId(
                            15,
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
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                                },
                                return_type: TypeId(
                                    21,
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
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                17,
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
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Reassignment(
                            TyReassignment {
                                lhs_base_name: BaseIdent {
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
                                lhs_type: TypeId(
                                    21,
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
                                                src (ptr): 0x00007fe08f9c9f10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                ),
                                                start: 148,
                                                end: 149,
                                                as_str(): "0",
                                            },
                                        },
                                        index_span: Span {
                                            src (ptr): 0x00007fe08f9c9f10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                            ),
                                            start: 148,
                                            end: 149,
                                            as_str(): "0",
                                        },
                                    },
                                ],
                                rhs: TyExpression {
                                    expression: Literal(
                                        U64(
                                            20,
                                        ),
                                    ),
                                    return_type: TypeId(
                                        25,
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
                        return_type: TypeId(
                            27,
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
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Reassignment(
                            TyReassignment {
                                lhs_base_name: BaseIdent {
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
                                                src (ptr): 0x00007fe08f9c9f10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                ),
                                                start: 173,
                                                end: 174,
                                                as_str(): "0",
                                            },
                                        },
                                        index_span: Span {
                                            src (ptr): 0x00007fe08f9c9f10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                            ),
                                            start: 173,
                                            end: 174,
                                            as_str(): "0",
                                        },
                                    },
                                ],
                                rhs: TyExpression {
                                    expression: ArrayIndex {
                                        prefix: TyExpression {
                                            expression: VariableExpression {
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
                                                span: Span {
                                                    src (ptr): 0x00007fe08f9c9f10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                    ),
                                                    start: 178,
                                                    end: 188,
                                                    as_str(): "my_array_1",
                                                },
                                                mutability: Mutable,
                                            },
                                            return_type: TypeId(
                                                33,
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
                                    return_type: TypeId(
                                        0,
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
                        return_type: TypeId(
                            36,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: ArrayIndex {
                            prefix: TyExpression {
                                expression: VariableExpression {
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
                                    span: Span {
                                        src (ptr): 0x00007fe08f9c9f10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                        ),
                                        start: 197,
                                        end: 207,
                                        as_str(): "my_array_0",
                                    },
                                    mutability: Mutable,
                                },
                                return_type: TypeId(
                                    39,
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
                            index: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    40,
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
                        return_type: TypeId(
                            0,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe08f9c9f10,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
        ),
        start: 9,
        end: 212,
        as_str(): "fn main() -> u64 {\n    let mut my_array_0: [u64; 1] = [1];\n    my_array_0[0] = 10;\n\n    let mut my_array_1: [u64; 1] = [1];\n    my_array_1[0] = 20;\n\n    my_array_0[0] = my_array_1[0];\n    my_array_0[0]\n}",
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
        src (ptr): 0x00007fe08f9c9f10,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
        ),
        start: 22,
        end: 25,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

