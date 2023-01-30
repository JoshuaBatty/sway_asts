TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe04b383550,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
            ),
            start: 391,
            end: 396,
            as_str(): "Dummy",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 403,
                    end: 409,
                    as_str(): "value1",
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
                src (ptr): 0x00007fe04b383550,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                ),
                start: 403,
                end: 414,
                as_str(): "value1: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe04b383550,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                ),
                start: 411,
                end: 414,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 420,
                    end: 426,
                    as_str(): "value2",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            span: Span {
                src (ptr): 0x00007fe04b383550,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                ),
                start: 420,
                end: 432,
                as_str(): "value2: bool",
            },
            type_span: Span {
                src (ptr): 0x00007fe04b383550,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                ),
                start: 428,
                end: 432,
                as_str(): "bool",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe04b383550,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
        ),
        start: 384,
        end: 435,
        as_str(): "struct Dummy {\n    value1: u64,\n    value2: bool,\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe04b383550,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
            ),
            start: 43,
            end: 57,
            as_str(): "gimme_a_struct",
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
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 391,
                                    end: 396,
                                    as_str(): "Dummy",
                                },
                                is_raw_ident: false,
                            },
                            fields: [
                                TyStructExpressionField {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 83,
                                            end: 89,
                                            as_str(): "value1",
                                        },
                                        is_raw_ident: false,
                                    },
                                    value: TyExpression {
                                        expression: Literal(
                                            U64(
                                                1,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 91,
                                            end: 92,
                                            as_str(): "1",
                                        },
                                    },
                                },
                                TyStructExpressionField {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 94,
                                            end: 100,
                                            as_str(): "value2",
                                        },
                                        is_raw_ident: false,
                                    },
                                    value: TyExpression {
                                        expression: Literal(
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 102,
                                            end: 106,
                                            as_str(): "true",
                                        },
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe04b383550,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                ),
                                start: 75,
                                end: 80,
                                as_str(): "Dummy",
                            },
                        },
                        return_type: TypeId(
                            7253,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04b383550,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                            ),
                            start: 75,
                            end: 108,
                            as_str(): "Dummy { value1: 1, value2: true }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 75,
                    end: 108,
                    as_str(): "Dummy { value1: 1, value2: true }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe04b383550,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
        ),
        start: 40,
        end: 110,
        as_str(): "fn gimme_a_struct() -> Dummy {\n    Dummy { value1: 1, value2: true }\n}",
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
        src (ptr): 0x00007fe04b383550,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
        ),
        start: 63,
        end: 68,
        as_str(): "Dummy",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe04b383550,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
            ),
            start: 357,
            end: 361,
            as_str(): "Data",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 369,
                    end: 374,
                    as_str(): "value",
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
                src (ptr): 0x00007fe04b383550,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                ),
                start: 369,
                end: 379,
                as_str(): "value: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe04b383550,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                ),
                start: 376,
                end: 379,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe04b383550,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
        ),
        start: 350,
        end: 382,
        as_str(): "struct Data { \n    value: u64,\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe04b383550,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
            ),
            start: 115,
            end: 119,
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
                                name_override_opt: Some(
                                    "__destructure_1",
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 139,
                                    end: 144,
                                    as_str(): "Dummy",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 166,
                                                end: 180,
                                                as_str(): "gimme_a_struct",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        548,
                                        Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 110,
                                            as_str(): "fn gimme_a_struct() -> Dummy {\n    Dummy { value1: 1, value2: true }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 166,
                                                end: 180,
                                                as_str(): "gimme_a_struct",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7253,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 166,
                                    end: 182,
                                    as_str(): "gimme_a_struct()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7253,
                            ),
                            type_ascription: TypeId(
                                7259,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 135,
                    end: 183,
                    as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 147,
                                    end: 153,
                                    as_str(): "value1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructFieldAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_1",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 139,
                                                    end: 144,
                                                    as_str(): "Dummy",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 139,
                                                end: 144,
                                                as_str(): "Dummy",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7253,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 135,
                                            end: 183,
                                            as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                                        },
                                    },
                                    field_to_access: TyStructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 403,
                                                end: 409,
                                                as_str(): "value1",
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
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 403,
                                            end: 414,
                                            as_str(): "value1: u64",
                                        },
                                        type_span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 411,
                                            end: 414,
                                            as_str(): "u64",
                                        },
                                        attributes: {},
                                    },
                                    field_instantiation_span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 147,
                                        end: 153,
                                        as_str(): "value1",
                                    },
                                    resolved_type_of_parent: TypeId(
                                        7253,
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 135,
                                    end: 183,
                                    as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7260,
                            ),
                            type_ascription: TypeId(
                                7260,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 135,
                    end: 183,
                    as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 155,
                                    end: 161,
                                    as_str(): "value2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructFieldAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_1",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 139,
                                                    end: 144,
                                                    as_str(): "Dummy",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 139,
                                                end: 144,
                                                as_str(): "Dummy",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7253,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 135,
                                            end: 183,
                                            as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                                        },
                                    },
                                    field_to_access: TyStructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 420,
                                                end: 426,
                                                as_str(): "value2",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_id: TypeId(
                                            71,
                                        ),
                                        initial_type_id: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 420,
                                            end: 432,
                                            as_str(): "value2: bool",
                                        },
                                        type_span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 428,
                                            end: 432,
                                            as_str(): "bool",
                                        },
                                        attributes: {},
                                    },
                                    field_instantiation_span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 155,
                                        end: 161,
                                        as_str(): "value2",
                                    },
                                    resolved_type_of_parent: TypeId(
                                        7253,
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 135,
                                    end: 183,
                                    as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                7262,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 135,
                    end: 183,
                    as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: Some(
                                    "__destructure_2",
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 192,
                                    end: 197,
                                    as_str(): "Dummy",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 226,
                                                end: 240,
                                                as_str(): "gimme_a_struct",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        550,
                                        Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 110,
                                            as_str(): "fn gimme_a_struct() -> Dummy {\n    Dummy { value1: 1, value2: true }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 226,
                                                end: 240,
                                                as_str(): "gimme_a_struct",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7253,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 226,
                                    end: 242,
                                    as_str(): "gimme_a_struct()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7253,
                            ),
                            type_ascription: TypeId(
                                7253,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 218,
                                    end: 223,
                                    as_str(): "Dummy",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 188,
                    end: 243,
                    as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 200,
                                    end: 206,
                                    as_str(): "value1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructFieldAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_2",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 192,
                                                    end: 197,
                                                    as_str(): "Dummy",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 192,
                                                end: 197,
                                                as_str(): "Dummy",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7253,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 188,
                                            end: 243,
                                            as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                                        },
                                    },
                                    field_to_access: TyStructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 403,
                                                end: 409,
                                                as_str(): "value1",
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
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 403,
                                            end: 414,
                                            as_str(): "value1: u64",
                                        },
                                        type_span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 411,
                                            end: 414,
                                            as_str(): "u64",
                                        },
                                        attributes: {},
                                    },
                                    field_instantiation_span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 200,
                                        end: 206,
                                        as_str(): "value1",
                                    },
                                    resolved_type_of_parent: TypeId(
                                        7253,
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 188,
                                    end: 243,
                                    as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7265,
                            ),
                            type_ascription: TypeId(
                                7265,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 188,
                    end: 243,
                    as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 208,
                                    end: 214,
                                    as_str(): "value2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructFieldAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_2",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 192,
                                                    end: 197,
                                                    as_str(): "Dummy",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 192,
                                                end: 197,
                                                as_str(): "Dummy",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7253,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 188,
                                            end: 243,
                                            as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                                        },
                                    },
                                    field_to_access: TyStructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 420,
                                                end: 426,
                                                as_str(): "value2",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_id: TypeId(
                                            71,
                                        ),
                                        initial_type_id: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 420,
                                            end: 432,
                                            as_str(): "value2: bool",
                                        },
                                        type_span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 428,
                                            end: 432,
                                            as_str(): "bool",
                                        },
                                        attributes: {},
                                    },
                                    field_instantiation_span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 208,
                                        end: 214,
                                        as_str(): "value2",
                                    },
                                    resolved_type_of_parent: TypeId(
                                        7253,
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 188,
                                    end: 243,
                                    as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                7267,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 188,
                    end: 243,
                    as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 252,
                                    end: 256,
                                    as_str(): "data",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 357,
                                            end: 361,
                                            as_str(): "Data",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 274,
                                                    end: 279,
                                                    as_str(): "value",
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
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 281,
                                                    end: 283,
                                                    as_str(): "42",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 259,
                                        end: 263,
                                        as_str(): "Data",
                                    },
                                },
                                return_type: TypeId(
                                    7271,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 259,
                                    end: 290,
                                    as_str(): "Data {\n        value: 42,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7271,
                            ),
                            type_ascription: TypeId(
                                7269,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 248,
                    end: 291,
                    as_str(): "let data = Data {\n        value: 42,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: Some(
                                    "__destructure_3",
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 300,
                                    end: 304,
                                    as_str(): "Data",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 252,
                                            end: 256,
                                            as_str(): "data",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 323,
                                        end: 327,
                                        as_str(): "data",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    7271,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 323,
                                    end: 327,
                                    as_str(): "data",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7271,
                            ),
                            type_ascription: TypeId(
                                7271,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 316,
                                    end: 320,
                                    as_str(): "Data",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 296,
                    end: 328,
                    as_str(): "let Data { value }: Data = data;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 307,
                                    end: 312,
                                    as_str(): "value",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructFieldAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_3",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 300,
                                                    end: 304,
                                                    as_str(): "Data",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 300,
                                                end: 304,
                                                as_str(): "Data",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7271,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 296,
                                            end: 328,
                                            as_str(): "let Data { value }: Data = data;",
                                        },
                                    },
                                    field_to_access: TyStructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe04b383550,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                ),
                                                start: 369,
                                                end: 374,
                                                as_str(): "value",
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
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 369,
                                            end: 379,
                                            as_str(): "value: u64",
                                        },
                                        type_span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 376,
                                            end: 379,
                                            as_str(): "u64",
                                        },
                                        attributes: {},
                                    },
                                    field_instantiation_span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 307,
                                        end: 312,
                                        as_str(): "value",
                                    },
                                    resolved_type_of_parent: TypeId(
                                        7271,
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 296,
                                    end: 328,
                                    as_str(): "let Data { value }: Data = data;",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7275,
                            ),
                            type_ascription: TypeId(
                                7275,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 296,
                    end: 328,
                    as_str(): "let Data { value }: Data = data;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
                            TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 307,
                                            end: 312,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04b383550,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                        ),
                                        start: 340,
                                        end: 345,
                                        as_str(): "value",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    7275,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 340,
                                    end: 345,
                                    as_str(): "value",
                                },
                            },
                        ),
                        return_type: TypeId(
                            7279,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04b383550,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                            ),
                            start: 333,
                            end: 345,
                            as_str(): "return value",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04b383550,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                    ),
                    start: 333,
                    end: 345,
                    as_str(): "return value",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe04b383550,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
        ),
        start: 112,
        end: 348,
        as_str(): "fn main() -> u64 {\n    let Dummy { value1, value2 } = gimme_a_struct();\n    let Dummy { value1, value2 }: Dummy = gimme_a_struct();\n    let data = Data {\n        value: 42,\n    };\n    let Data { value }: Data = data;\n    return value;\n}",
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
        src (ptr): 0x00007fe04b383550,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
        ),
        start: 125,
        end: 128,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

