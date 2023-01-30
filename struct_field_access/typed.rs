TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe04b55c700,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
            ),
            start: 286,
            end: 290,
            as_str(): "Data",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe04b55c700,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                    ),
                    start: 297,
                    end: 310,
                    as_str(): "uselessnumber",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            span: Span {
                src (ptr): 0x00007fe04b55c700,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                ),
                start: 297,
                end: 315,
                as_str(): "uselessnumber: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe04b55c700,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                ),
                start: 312,
                end: 315,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe04b55c700,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
        ),
        start: 279,
        end: 318,
        as_str(): "struct Data {\n    uselessnumber: u64,\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe04b55c700,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
            ),
            start: 323,
            end: 333,
            as_str(): "ret_struct",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: StructExpression {
                            struct_name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04b55c700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                    ),
                                    start: 286,
                                    end: 290,
                                    as_str(): "Data",
                                },
                                is_raw_ident: false,
                            },
                            fields: [
                                TyStructExpressionField {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 365,
                                            end: 378,
                                            as_str(): "uselessnumber",
                                        },
                                        is_raw_ident: false,
                                    },
                                    value: TyExpression {
                                        expression: Literal(
                                            U64(
                                                44,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 380,
                                            end: 382,
                                            as_str(): "44",
                                        },
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe04b55c700,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                ),
                                start: 350,
                                end: 354,
                                as_str(): "Data",
                            },
                        },
                        return_type: TypeId(
                            7253,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04b55c700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                            ),
                            start: 350,
                            end: 389,
                            as_str(): "Data {\n        uselessnumber: 44,\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04b55c700,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                    ),
                    start: 350,
                    end: 389,
                    as_str(): "Data {\n        uselessnumber: 44,\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe04b55c700,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
        ),
        start: 320,
        end: 391,
        as_str(): "fn ret_struct() -> Data {\n    Data {\n        uselessnumber: 44,\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        7253,
    ),
    initial_return_type: TypeId(
        7252,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe04b55c700,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
        ),
        start: 339,
        end: 343,
        as_str(): "Data",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe04b55c700,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
            ),
            start: 94,
            end: 98,
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
                                    src (ptr): 0x00007fe04b55c700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                    ),
                                    start: 122,
                                    end: 126,
                                    as_str(): "data",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 286,
                                            end: 290,
                                            as_str(): "Data",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04b55c700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                    ),
                                                    start: 144,
                                                    end: 157,
                                                    as_str(): "uselessnumber",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        42,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b55c700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                    ),
                                                    start: 159,
                                                    end: 161,
                                                    as_str(): "42",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe04b55c700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                        ),
                                        start: 129,
                                        end: 133,
                                        as_str(): "Data",
                                    },
                                },
                                return_type: TypeId(
                                    7253,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b55c700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                    ),
                                    start: 129,
                                    end: 168,
                                    as_str(): "Data {\n        uselessnumber: 42,\n    }",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                7253,
                            ),
                            type_ascription: TypeId(
                                7258,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04b55c700,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                    ),
                    start: 114,
                    end: 169,
                    as_str(): "let mut data = Data {\n        uselessnumber: 42,\n    };",
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
                                        src (ptr): 0x00007fe04b55c700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                        ),
                                        start: 174,
                                        end: 178,
                                        as_str(): "data",
                                    },
                                    is_raw_ident: false,
                                },
                                lhs_type: TypeId(
                                    7253,
                                ),
                                lhs_indices: [
                                    StructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe04b55c700,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                ),
                                                start: 179,
                                                end: 192,
                                                as_str(): "uselessnumber",
                                            },
                                            is_raw_ident: false,
                                        },
                                    },
                                ],
                                rhs: TyExpression {
                                    expression: Literal(
                                        U64(
                                            43,
                                        ),
                                    ),
                                    return_type: TypeId(
                                        7264,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe04b55c700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                        ),
                                        start: 195,
                                        end: 197,
                                        as_str(): "43",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            7266,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04b55c700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                            ),
                            start: 174,
                            end: 197,
                            as_str(): "data.uselessnumber = 43",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04b55c700,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                    ),
                    start: 174,
                    end: 197,
                    as_str(): "data.uselessnumber = 43",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04b55c700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                    ),
                                    start: 208,
                                    end: 213,
                                    as_str(): "other",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructFieldAccess {
                                    prefix: TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe04b55c700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                        ),
                                                        start: 216,
                                                        end: 226,
                                                        as_str(): "ret_struct",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: false,
                                            },
                                            contract_call_params: {},
                                            arguments: [],
                                            function_decl_id: DeclId(
                                                547,
                                                Span {
                                                    src (ptr): 0x00007fe04b55c700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                    ),
                                                    start: 320,
                                                    end: 391,
                                                    as_str(): "fn ret_struct() -> Data {\n    Data {\n        uselessnumber: 44,\n    }\n}",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04b55c700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                        ),
                                                        start: 216,
                                                        end: 226,
                                                        as_str(): "ret_struct",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            7253,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 216,
                                            end: 228,
                                            as_str(): "ret_struct()",
                                        },
                                    },
                                    field_to_access: TyStructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe04b55c700,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                ),
                                                start: 297,
                                                end: 310,
                                                as_str(): "uselessnumber",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_id: TypeId(
                                            21,
                                        ),
                                        initial_type_id: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 297,
                                            end: 315,
                                            as_str(): "uselessnumber: u64",
                                        },
                                        type_span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 312,
                                            end: 315,
                                            as_str(): "u64",
                                        },
                                        attributes: {},
                                    },
                                    field_instantiation_span: Span {
                                        src (ptr): 0x00007fe04b55c700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                        ),
                                        start: 229,
                                        end: 242,
                                        as_str(): "uselessnumber",
                                    },
                                    resolved_type_of_parent: TypeId(
                                        7253,
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b55c700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                    ),
                                    start: 216,
                                    end: 242,
                                    as_str(): "ret_struct().uselessnumber",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7267,
                            ),
                            type_ascription: TypeId(
                                7267,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04b55c700,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                    ),
                    start: 204,
                    end: 243,
                    as_str(): "let other = ret_struct().uselessnumber;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
                            TyExpression {
                                expression: StructFieldAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04b55c700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                    ),
                                                    start: 122,
                                                    end: 126,
                                                    as_str(): "data",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe04b55c700,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                ),
                                                start: 256,
                                                end: 260,
                                                as_str(): "data",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            7253,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 256,
                                            end: 260,
                                            as_str(): "data",
                                        },
                                    },
                                    field_to_access: TyStructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe04b55c700,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                ),
                                                start: 297,
                                                end: 310,
                                                as_str(): "uselessnumber",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_id: TypeId(
                                            21,
                                        ),
                                        initial_type_id: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 297,
                                            end: 315,
                                            as_str(): "uselessnumber: u64",
                                        },
                                        type_span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 312,
                                            end: 315,
                                            as_str(): "u64",
                                        },
                                        attributes: {},
                                    },
                                    field_instantiation_span: Span {
                                        src (ptr): 0x00007fe04b55c700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                        ),
                                        start: 261,
                                        end: 274,
                                        as_str(): "uselessnumber",
                                    },
                                    resolved_type_of_parent: TypeId(
                                        7253,
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b55c700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                    ),
                                    start: 256,
                                    end: 274,
                                    as_str(): "data.uselessnumber",
                                },
                            },
                        ),
                        return_type: TypeId(
                            7272,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04b55c700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                            ),
                            start: 249,
                            end: 274,
                            as_str(): "return data.uselessnumber",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04b55c700,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                    ),
                    start: 249,
                    end: 274,
                    as_str(): "return data.uselessnumber",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe04b55c700,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
        ),
        start: 91,
        end: 277,
        as_str(): "fn main() -> u64 {\n    let mut data = Data {\n        uselessnumber: 42,\n    };\n    data.uselessnumber = 43;\n\n    let other = ret_struct().uselessnumber;\n\n    return data.uselessnumber;\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe04b55c700,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
        ),
        start: 104,
        end: 107,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

