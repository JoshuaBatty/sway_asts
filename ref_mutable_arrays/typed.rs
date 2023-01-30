TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe05c5aa670,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
            ),
            start: 108,
            end: 127,
            as_str(): "takes_ref_mut_array",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Reassignment(
                            TyReassignment {
                                lhs_base_name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe05c5aa670,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                        ),
                                        start: 157,
                                        end: 160,
                                        as_str(): "arr",
                                    },
                                    is_raw_ident: false,
                                },
                                lhs_type: TypeId(
                                    5,
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
                                                src (ptr): 0x00007fe05c5aa670,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                ),
                                                start: 161,
                                                end: 162,
                                                as_str(): "0",
                                            },
                                        },
                                        index_span: Span {
                                            src (ptr): 0x00007fe05c5aa670,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                            ),
                                            start: 161,
                                            end: 162,
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
                                        11,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe05c5aa670,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                        ),
                                        start: 166,
                                        end: 168,
                                        as_str(): "10",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            13,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe05c5aa670,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                            ),
                            start: 157,
                            end: 168,
                            as_str(): "arr[0] = 10",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe05c5aa670,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                    ),
                    start: 157,
                    end: 168,
                    as_str(): "arr[0] = 10",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe05c5aa670,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                    ),
                    start: 136,
                    end: 139,
                    as_str(): "arr",
                },
                is_raw_ident: false,
            },
            is_reference: true,
            is_mutable: true,
            mutability_span: Span {
                src (ptr): 0x00007fe05c5aa670,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                ),
                start: 128,
                end: 135,
                as_str(): "ref mut",
            },
            type_id: TypeId(
                5,
            ),
            initial_type_id: TypeId(
                4,
            ),
            type_span: Span {
                src (ptr): 0x00007fe05c5aa670,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                ),
                start: 141,
                end: 149,
                as_str(): "[u64; 1]",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe05c5aa670,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
        ),
        start: 105,
        end: 171,
        as_str(): "fn takes_ref_mut_array(ref mut arr: [u64; 1]) {\n    arr[0] = 10;\n}",
    },
    attributes: {},
    return_type: TypeId(
        7,
    ),
    initial_return_type: TypeId(
        6,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe05c5aa670,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
        ),
        start: 105,
        end: 150,
        as_str(): "fn takes_ref_mut_array(ref mut arr: [u64; 1])",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe05c5aa670,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
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
                                    src (ptr): 0x00007fe05c5aa670,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                    ),
                                    start: 40,
                                    end: 43,
                                    as_str(): "arr",
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
                                                src (ptr): 0x00007fe05c5aa670,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                ),
                                                start: 57,
                                                end: 58,
                                                as_str(): "1",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05c5aa670,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                    ),
                                    start: 56,
                                    end: 59,
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
                                    src (ptr): 0x00007fe05c5aa670,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                    ),
                                    start: 45,
                                    end: 53,
                                    as_str(): "[u64; 1]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe05c5aa670,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                    ),
                    start: 32,
                    end: 60,
                    as_str(): "let mut arr: [u64; 1] = [1];",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe05c5aa670,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                        ),
                                        start: 65,
                                        end: 84,
                                        as_str(): "takes_ref_mut_array",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05c5aa670,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                            ),
                                            start: 136,
                                            end: 139,
                                            as_str(): "arr",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05c5aa670,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                    ),
                                                    start: 40,
                                                    end: 43,
                                                    as_str(): "arr",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe05c5aa670,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                ),
                                                start: 85,
                                                end: 88,
                                                as_str(): "arr",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            24,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05c5aa670,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                            ),
                                            start: 85,
                                            end: 88,
                                            as_str(): "arr",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                2,
                                Span {
                                    src (ptr): 0x00007fe05c5aa670,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                    ),
                                    start: 105,
                                    end: 171,
                                    as_str(): "fn takes_ref_mut_array(ref mut arr: [u64; 1]) {\n    arr[0] = 10;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe05c5aa670,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                        ),
                                        start: 65,
                                        end: 84,
                                        as_str(): "takes_ref_mut_array",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            25,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe05c5aa670,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                            ),
                            start: 65,
                            end: 89,
                            as_str(): "takes_ref_mut_array(arr)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe05c5aa670,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                    ),
                    start: 65,
                    end: 89,
                    as_str(): "takes_ref_mut_array(arr)",
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
                                            src (ptr): 0x00007fe05c5aa670,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 43,
                                            as_str(): "arr",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05c5aa670,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 98,
                                        as_str(): "arr",
                                    },
                                    mutability: Mutable,
                                },
                                return_type: TypeId(
                                    28,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05c5aa670,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                    ),
                                    start: 95,
                                    end: 98,
                                    as_str(): "arr",
                                },
                            },
                            index: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    29,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05c5aa670,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                    ),
                                    start: 99,
                                    end: 100,
                                    as_str(): "0",
                                },
                            },
                        },
                        return_type: TypeId(
                            0,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe05c5aa670,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                            ),
                            start: 95,
                            end: 101,
                            as_str(): "arr[0]",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe05c5aa670,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                    ),
                    start: 95,
                    end: 101,
                    as_str(): "arr[0]",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe05c5aa670,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
        ),
        start: 9,
        end: 103,
        as_str(): "fn main() -> u64 {\n    let mut arr: [u64; 1] = [1];\n    takes_ref_mut_array(arr);\n    arr[0]\n}",
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
        src (ptr): 0x00007fe05c5aa670,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
        ),
        start: 22,
        end: 25,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

