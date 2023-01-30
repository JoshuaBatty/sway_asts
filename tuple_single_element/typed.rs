
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a18734b80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
            ),
            start: 42,
            end: 43,
            as_str(): "S",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007f8a18734b80,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                    ),
                    start: 50,
                    end: 51,
                    as_str(): "t",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31631,
            ),
            initial_type_id: TypeId(
                31630,
            ),
            span: Span {
                src (ptr): 0x00007f8a18734b80,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                ),
                start: 50,
                end: 59,
                as_str(): "t: (u64,)",
            },
            type_span: Span {
                src (ptr): 0x00007f8a18734b80,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                ),
                start: 53,
                end: 59,
                as_str(): "(u64,)",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007f8a18734b80,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
        ),
        start: 35,
        end: 61,
        as_str(): "struct S {\n    t: (u64,)\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a18734b80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
            ),
            start: 66,
            end: 70,
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
                                    src (ptr): 0x00007f8a18734b80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                    ),
                                    start: 90,
                                    end: 91,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a18734b80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                            ),
                                            start: 42,
                                            end: 43,
                                            as_str(): "S",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a18734b80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                    ),
                                                    start: 106,
                                                    end: 107,
                                                    as_str(): "t",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Tuple {
                                                    fields: [
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    2,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a18734b80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                ),
                                                                start: 110,
                                                                end: 111,
                                                                as_str(): "2",
                                                            },
                                                        },
                                                    ],
                                                },
                                                return_type: TypeId(
                                                    31640,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a18734b80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                    ),
                                                    start: 109,
                                                    end: 113,
                                                    as_str(): "(2,)",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007f8a18734b80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                        ),
                                        start: 94,
                                        end: 95,
                                        as_str(): "S",
                                    },
                                },
                                return_type: TypeId(
                                    31635,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a18734b80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                    ),
                                    start: 94,
                                    end: 119,
                                    as_str(): "S {\n        t: (2,)\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31635,
                            ),
                            type_ascription: TypeId(
                                31633,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a18734b80,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                    ),
                    start: 86,
                    end: 120,
                    as_str(): "let a = S {\n        t: (2,)\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a18734b80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                    ),
                                    start: 129,
                                    end: 130,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Declaration(
                                                    VariableDeclaration(
                                                        TyVariableDeclaration {
                                                            name: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "__match_return_var_name_1",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a18734b80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                    ),
                                                                    start: 139,
                                                                    end: 140,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            body: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a18734b80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                            ),
                                                                            start: 90,
                                                                            end: 91,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a18734b80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                        ),
                                                                        start: 139,
                                                                        end: 140,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31635,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a18734b80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                    ),
                                                                    start: 139,
                                                                    end: 140,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                31635,
                                                            ),
                                                            type_ascription: TypeId(
                                                                31642,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a18734b80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                    ),
                                                    start: 133,
                                                    end: 170,
                                                    as_str(): "match a {\n        S { t } => t,\n    }",
                                                },
                                            },
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: Declaration(
                                                                            VariableDeclaration(
                                                                                TyVariableDeclaration {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a18734b80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                            ),
                                                                                            start: 155,
                                                                                            end: 156,
                                                                                            as_str(): "t",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    body: TyExpression {
                                                                                        expression: StructFieldAccess {
                                                                                            prefix: TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "__match_return_var_name_1",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a18734b80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                            ),
                                                                                                            start: 139,
                                                                                                            end: 140,
                                                                                                            as_str(): "a",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a18734b80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                        ),
                                                                                                        start: 139,
                                                                                                        end: 140,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    31635,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                    ),
                                                                                                    start: 139,
                                                                                                    end: 140,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: TyStructField {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a18734b80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                        ),
                                                                                                        start: 50,
                                                                                                        end: 51,
                                                                                                        as_str(): "t",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_id: TypeId(
                                                                                                    31631,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    31630,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                    ),
                                                                                                    start: 50,
                                                                                                    end: 59,
                                                                                                    as_str(): "t: (u64,)",
                                                                                                },
                                                                                                type_span: Span {
                                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                    ),
                                                                                                    start: 53,
                                                                                                    end: 59,
                                                                                                    as_str(): "(u64,)",
                                                                                                },
                                                                                                attributes: {},
                                                                                            },
                                                                                            field_instantiation_span: Span {
                                                                                                src (ptr): 0x00007f8a18734b80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                ),
                                                                                                start: 155,
                                                                                                end: 156,
                                                                                                as_str(): "t",
                                                                                            },
                                                                                            resolved_type_of_parent: TypeId(
                                                                                                31635,
                                                                                            ),
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a18734b80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                            ),
                                                                                            start: 155,
                                                                                            end: 156,
                                                                                            as_str(): "t",
                                                                                        },
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                    return_type: TypeId(
                                                                                        31631,
                                                                                    ),
                                                                                    type_ascription: TypeId(
                                                                                        31631,
                                                                                    ),
                                                                                    type_ascription_span: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a18734b80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                            ),
                                                                            start: 155,
                                                                            end: 156,
                                                                            as_str(): "t",
                                                                        },
                                                                    },
                                                                    TyAstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            TyExpression {
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a18734b80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                            ),
                                                                                            start: 155,
                                                                                            end: 156,
                                                                                            as_str(): "t",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a18734b80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                        ),
                                                                                        start: 162,
                                                                                        end: 163,
                                                                                        as_str(): "t",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    31645,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                    ),
                                                                                    start: 162,
                                                                                    end: 163,
                                                                                    as_str(): "t",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a18734b80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                            ),
                                                                            start: 162,
                                                                            end: 163,
                                                                            as_str(): "t",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31646,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 162,
                                                            end: 163,
                                                            as_str(): "t",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a18734b80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                    ),
                                                    start: 133,
                                                    end: 170,
                                                    as_str(): "match a {\n        S { t } => t,\n    }",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    31647,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a18734b80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                    ),
                                    start: 133,
                                    end: 170,
                                    as_str(): "match a {\n        S { t } => t,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31647,
                            ),
                            type_ascription: TypeId(
                                31641,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a18734b80,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                    ),
                    start: 125,
                    end: 171,
                    as_str(): "let b = match a {\n        S { t } => t,\n    };",
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
                                        src (ptr): 0x00007f8a18734b80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                        ),
                                        start: 176,
                                        end: 182,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007f8a2920d290,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 187,
                                                            end: 189,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 187,
                                                            end: 189,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007f8a18734b80,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                        ),
                                                        start: 187,
                                                        end: 189,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28b609c0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: TupleElemAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a18734b80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                            ),
                                                                            start: 129,
                                                                            end: 130,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a18734b80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                        ),
                                                                        start: 183,
                                                                        end: 184,
                                                                        as_str(): "b",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31652,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a18734b80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                    ),
                                                                    start: 183,
                                                                    end: 184,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                            elem_to_access_num: 0,
                                                            resolved_type_of_parent: TypeId(
                                                                31652,
                                                            ),
                                                            elem_to_access_span: Span {
                                                                src (ptr): 0x00007f8a18734b80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                ),
                                                                start: 185,
                                                                end: 186,
                                                                as_str(): "0",
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 183,
                                                            end: 186,
                                                            as_str(): "b.0",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28b609c0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 190,
                                                            end: 191,
                                                            as_str(): "2",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13316,
                                                Span {
                                                    src (ptr): 0x00007f8a28b609c0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a18734b80,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                        ),
                                                        start: 187,
                                                        end: 189,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a18734b80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                            ),
                                            start: 183,
                                            end: 191,
                                            as_str(): "b.0 == 2",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13317,
                                Span {
                                    src (ptr): 0x00007f8a2920d290,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007f8a18734b80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                        ),
                                        start: 176,
                                        end: 182,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31655,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a18734b80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                            ),
                            start: 176,
                            end: 192,
                            as_str(): "assert(b.0 == 2)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a18734b80,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                    ),
                    start: 176,
                    end: 192,
                    as_str(): "assert(b.0 == 2)",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                1,
                            ),
                        ),
                        return_type: TypeId(
                            31656,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a18734b80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                            ),
                            start: 199,
                            end: 200,
                            as_str(): "1",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a18734b80,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                    ),
                    start: 199,
                    end: 200,
                    as_str(): "1",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a18734b80,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
        ),
        start: 63,
        end: 202,
        as_str(): "fn main() -> u64 {\n    let a = S {\n        t: (2,)\n    };\n    let b = match a {\n        S { t } => t,\n    };\n    assert(b.0 == 2);\n\n    1\n}",
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
        src (ptr): 0x00007f8a18734b80,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
        ),
        start: 76,
        end: 79,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

