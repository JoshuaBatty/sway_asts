TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe096cd2c70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
            ),
            start: 16,
            end: 19,
            as_str(): "Foo",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe096cd2c70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                    ),
                    start: 26,
                    end: 31,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                3,
            ),
            initial_type_id: TypeId(
                3,
            ),
            span: Span {
                src (ptr): 0x00007fe096cd2c70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                ),
                start: 26,
                end: 36,
                as_str(): "value: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe096cd2c70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                ),
                start: 33,
                end: 36,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe096cd2c70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
        ),
        start: 9,
        end: 38,
        as_str(): "struct Foo {\n    value: u64\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe096cd2c70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
            ),
            start: 43,
            end: 47,
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
                                    src (ptr): 0x00007fe096cd2c70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                    ),
                                    start: 71,
                                    end: 79,
                                    as_str(): "my_array",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Array {
                                    contents: [
                                        TyExpression {
                                            expression: StructExpression {
                                                struct_name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe096cd2c70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                        ),
                                                        start: 16,
                                                        end: 19,
                                                        as_str(): "Foo",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                fields: [
                                                    TyStructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe096cd2c70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                ),
                                                                start: 97,
                                                                end: 102,
                                                                as_str(): "value",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    10,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                3,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe096cd2c70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                ),
                                                                start: 104,
                                                                end: 106,
                                                                as_str(): "10",
                                                            },
                                                        },
                                                    },
                                                ],
                                                span: Span {
                                                    src (ptr): 0x00007fe096cd2c70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                    ),
                                                    start: 93,
                                                    end: 96,
                                                    as_str(): "Foo",
                                                },
                                            },
                                            return_type: TypeId(
                                                6,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe096cd2c70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                ),
                                                start: 93,
                                                end: 107,
                                                as_str(): "Foo{value: 10}",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    13,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe096cd2c70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                    ),
                                    start: 92,
                                    end: 108,
                                    as_str(): "[Foo{value: 10}]",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                13,
                            ),
                            type_ascription: TypeId(
                                7,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe096cd2c70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                    ),
                                    start: 81,
                                    end: 89,
                                    as_str(): "[Foo; 1]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe096cd2c70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                    ),
                    start: 63,
                    end: 109,
                    as_str(): "let mut my_array: [Foo; 1] = [Foo{value: 10}];",
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
                                        src (ptr): 0x00007fe096cd2c70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                        ),
                                        start: 114,
                                        end: 122,
                                        as_str(): "my_array",
                                    },
                                    is_raw_ident: false,
                                },
                                lhs_type: TypeId(
                                    13,
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
                                                src (ptr): 0x00007fe096cd2c70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                ),
                                                start: 123,
                                                end: 124,
                                                as_str(): "0",
                                            },
                                        },
                                        index_span: Span {
                                            src (ptr): 0x00007fe096cd2c70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                            ),
                                            start: 123,
                                            end: 124,
                                            as_str(): "0",
                                        },
                                    },
                                ],
                                rhs: TyExpression {
                                    expression: StructExpression {
                                        struct_name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe096cd2c70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                ),
                                                start: 16,
                                                end: 19,
                                                as_str(): "Foo",
                                            },
                                            is_raw_ident: false,
                                        },
                                        fields: [
                                            TyStructExpressionField {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe096cd2c70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                        ),
                                                        start: 132,
                                                        end: 137,
                                                        as_str(): "value",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                value: TyExpression {
                                                    expression: Literal(
                                                        U64(
                                                            20,
                                                        ),
                                                    ),
                                                    return_type: TypeId(
                                                        3,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe096cd2c70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                        ),
                                                        start: 139,
                                                        end: 141,
                                                        as_str(): "20",
                                                    },
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe096cd2c70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 131,
                                            as_str(): "Foo",
                                        },
                                    },
                                    return_type: TypeId(
                                        6,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe096cd2c70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                        ),
                                        start: 128,
                                        end: 142,
                                        as_str(): "Foo{value: 20}",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe096cd2c70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                            ),
                            start: 114,
                            end: 142,
                            as_str(): "my_array[0] = Foo{value: 20}",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe096cd2c70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                    ),
                    start: 114,
                    end: 142,
                    as_str(): "my_array[0] = Foo{value: 20}",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: StructFieldAccess {
                            prefix: TyExpression {
                                expression: ArrayIndex {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe096cd2c70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                    ),
                                                    start: 71,
                                                    end: 79,
                                                    as_str(): "my_array",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe096cd2c70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                ),
                                                start: 148,
                                                end: 156,
                                                as_str(): "my_array",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            25,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe096cd2c70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                            ),
                                            start: 148,
                                            end: 156,
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
                                            26,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe096cd2c70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                            ),
                                            start: 157,
                                            end: 158,
                                            as_str(): "0",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    6,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe096cd2c70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                    ),
                                    start: 148,
                                    end: 159,
                                    as_str(): "my_array[0]",
                                },
                            },
                            field_to_access: TyStructField {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe096cd2c70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 31,
                                        as_str(): "value",
                                    },
                                    is_raw_ident: false,
                                },
                                type_id: TypeId(
                                    3,
                                ),
                                initial_type_id: TypeId(
                                    3,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe096cd2c70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                    ),
                                    start: 26,
                                    end: 36,
                                    as_str(): "value: u64",
                                },
                                type_span: Span {
                                    src (ptr): 0x00007fe096cd2c70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                    ),
                                    start: 33,
                                    end: 36,
                                    as_str(): "u64",
                                },
                                attributes: {},
                            },
                            field_instantiation_span: Span {
                                src (ptr): 0x00007fe096cd2c70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                ),
                                start: 160,
                                end: 165,
                                as_str(): "value",
                            },
                            resolved_type_of_parent: TypeId(
                                6,
                            ),
                        },
                        return_type: TypeId(
                            3,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe096cd2c70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                            ),
                            start: 148,
                            end: 165,
                            as_str(): "my_array[0].value",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe096cd2c70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                    ),
                    start: 148,
                    end: 165,
                    as_str(): "my_array[0].value",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe096cd2c70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
        ),
        start: 40,
        end: 167,
        as_str(): "fn main() -> u64 {\n    let mut my_array: [Foo; 1] = [Foo{value: 10}];\n    my_array[0] = Foo{value: 20};\n    my_array[0].value\n}",
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
        src (ptr): 0x00007fe096cd2c70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
        ),
        start: 53,
        end: 56,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

