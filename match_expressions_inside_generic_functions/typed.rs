

TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 53,
            end: 64,
            as_str(): "third_match",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
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
                                                        name_override_opt: Some(
                                                            "__match_return_var_name_1",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 94,
                                                            end: 99,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 68,
                                                                    end: 73,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 94,
                                                                end: 99,
                                                                as_str(): "value",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31642,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 94,
                                                            end: 99,
                                                            as_str(): "value",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        31642,
                                                    ),
                                                    type_ascription: TypeId(
                                                        31644,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 88,
                                            end: 121,
                                            as_str(): "match value {\n    foo => 5u8,\n  }",
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 106,
                                                                                    end: 109,
                                                                                    as_str(): "foo",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            body: TyExpression {
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "__match_return_var_name_1",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 94,
                                                                                            end: 99,
                                                                                            as_str(): "value",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 94,
                                                                                        end: 99,
                                                                                        as_str(): "value",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    31642,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 94,
                                                                                    end: 99,
                                                                                    as_str(): "value",
                                                                                },
                                                                            },
                                                                            mutability: Immutable,
                                                                            return_type: TypeId(
                                                                                31642,
                                                                            ),
                                                                            type_ascription: TypeId(
                                                                                31642,
                                                                            ),
                                                                            type_ascription_span: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 106,
                                                                    end: 109,
                                                                    as_str(): "foo",
                                                                },
                                                            },
                                                            TyAstNode {
                                                                content: ImplicitReturnExpression(
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U8(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 113,
                                                                            end: 116,
                                                                            as_str(): "5u8",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 113,
                                                                    end: 116,
                                                                    as_str(): "5u8",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    50,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 113,
                                                    end: 116,
                                                    as_str(): "5u8",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 88,
                                            end: 121,
                                            as_str(): "match value {\n    foo => 5u8,\n  }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            50,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 88,
                            end: 121,
                            as_str(): "match value {\n    foo => 5u8,\n  }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 88,
                    end: 121,
                    as_str(): "match value {\n    foo => 5u8,\n  }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 68,
                    end: 73,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                31642,
            ),
            initial_type_id: TypeId(
                31643,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09f0b11e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                ),
                start: 75,
                end: 76,
                as_str(): "A",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 50,
        end: 123,
        as_str(): "fn third_match<A>(value: A) -> u8 {\n  match value {\n    foo => 5u8,\n  }\n}",
    },
    attributes: {},
    return_type: TypeId(
        50,
    ),
    initial_return_type: TypeId(
        50,
    ),
    type_parameters: [
        A: TypeId(31642),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 81,
        end: 83,
        as_str(): "u8",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 128,
            end: 140,
            as_str(): "second_match",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
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
                                                        name_override_opt: Some(
                                                            "__match_return_var_name_2",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 190,
                                                            as_str(): "third_match(value)",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 172,
                                                                        end: 183,
                                                                        as_str(): "third_match",
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 68,
                                                                            end: 73,
                                                                            as_str(): "value",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 144,
                                                                                    end: 149,
                                                                                    as_str(): "value",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 184,
                                                                                end: 189,
                                                                                as_str(): "value",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31649,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 184,
                                                                            end: 189,
                                                                            as_str(): "value",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13316,
                                                                Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 50,
                                                                    end: 123,
                                                                    as_str(): "fn third_match<A>(value: A) -> u8 {\n  match value {\n    foo => 5u8,\n  }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 172,
                                                                        end: 183,
                                                                        as_str(): "third_match",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 190,
                                                            as_str(): "third_match(value)",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        31651,
                                                    ),
                                                    type_ascription: TypeId(
                                                        31651,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 166,
                                            end: 301,
                                            as_str(): "match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }",
                                        },
                                    },
                                    TyAstNode {
                                        content: ImplicitReturnExpression(
                                            TyExpression {
                                                expression: IfExp {
                                                    condition: TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 172,
                                                                            end: 200,
                                                                            as_str(): "third_match(value) {\n    1u8",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 172,
                                                                            end: 200,
                                                                            as_str(): "third_match(value) {\n    1u8",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "eq",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 172,
                                                                        end: 200,
                                                                        as_str(): "third_match(value) {\n    1u8",
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
                                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3297,
                                                                            end: 3301,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "__match_return_var_name_2",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 190,
                                                                                    as_str(): "third_match(value)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 172,
                                                                                end: 190,
                                                                                as_str(): "third_match(value)",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31651,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 172,
                                                                            end: 190,
                                                                            as_str(): "third_match(value)",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3303,
                                                                            end: 3308,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U8(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31651,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 197,
                                                                            end: 200,
                                                                            as_str(): "1u8",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13321,
                                                                Span {
                                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 3291,
                                                                    end: 3357,
                                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: None,
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 200,
                                                            as_str(): "third_match(value) {\n    1u8",
                                                        },
                                                    },
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            TyExpression {
                                                                                expression: Literal(
                                                                                    Boolean(
                                                                                        false,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 204,
                                                                                    end: 209,
                                                                                    as_str(): "false",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 204,
                                                                            end: 209,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 204,
                                                            end: 209,
                                                            as_str(): "false",
                                                        },
                                                    },
                                                    else: Some(
                                                        TyExpression {
                                                            expression: IfExp {
                                                                condition: TyExpression {
                                                                    expression: FunctionApplication {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 218,
                                                                                        as_str(): "third_match(value) {\n    1u8 => false,\n    2u8",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 218,
                                                                                        as_str(): "third_match(value) {\n    1u8 => false,\n    2u8",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "eq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 218,
                                                                                    as_str(): "third_match(value) {\n    1u8 => false,\n    2u8",
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
                                                                                        src (ptr): 0x00007fe0ae6d93e0,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                        ),
                                                                                        start: 3297,
                                                                                        end: 3301,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "__match_return_var_name_2",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 172,
                                                                                                end: 190,
                                                                                                as_str(): "third_match(value)",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 172,
                                                                                            end: 190,
                                                                                            as_str(): "third_match(value)",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        31651,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 190,
                                                                                        as_str(): "third_match(value)",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ae6d93e0,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                        ),
                                                                                        start: 3303,
                                                                                        end: 3308,
                                                                                        as_str(): "other",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                TyExpression {
                                                                                    expression: Literal(
                                                                                        U8(
                                                                                            2,
                                                                                        ),
                                                                                    ),
                                                                                    return_type: TypeId(
                                                                                        31651,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 215,
                                                                                        end: 218,
                                                                                        as_str(): "2u8",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        function_decl_id: DeclId(
                                                                            13320,
                                                                            Span {
                                                                                src (ptr): 0x00007fe0ae6d93e0,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 3291,
                                                                                end: 3357,
                                                                                as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                            },
                                                                        ),
                                                                        self_state_idx: None,
                                                                        selector: None,
                                                                        type_binding: None,
                                                                    },
                                                                    return_type: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 172,
                                                                        end: 218,
                                                                        as_str(): "third_match(value) {\n    1u8 => false,\n    2u8",
                                                                    },
                                                                },
                                                                then: TyExpression {
                                                                    expression: CodeBlock(
                                                                        TyCodeBlock {
                                                                            contents: [
                                                                                TyAstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        TyExpression {
                                                                                            expression: Literal(
                                                                                                Boolean(
                                                                                                    false,
                                                                                                ),
                                                                                            ),
                                                                                            return_type: TypeId(
                                                                                                71,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 222,
                                                                                                end: 227,
                                                                                                as_str(): "false",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 222,
                                                                                        end: 227,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    return_type: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 222,
                                                                        end: 227,
                                                                        as_str(): "false",
                                                                    },
                                                                },
                                                                else: Some(
                                                                    TyExpression {
                                                                        expression: IfExp {
                                                                            condition: TyExpression {
                                                                                expression: FunctionApplication {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "core",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 172,
                                                                                                    end: 236,
                                                                                                    as_str(): "third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 172,
                                                                                                    end: 236,
                                                                                                    as_str(): "third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "eq",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 172,
                                                                                                end: 236,
                                                                                                as_str(): "third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8",
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
                                                                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                    ),
                                                                                                    start: 3297,
                                                                                                    end: 3301,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "__match_return_var_name_2",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 172,
                                                                                                            end: 190,
                                                                                                            as_str(): "third_match(value)",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 172,
                                                                                                        end: 190,
                                                                                                        as_str(): "third_match(value)",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    31651,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 172,
                                                                                                    end: 190,
                                                                                                    as_str(): "third_match(value)",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        (
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                    ),
                                                                                                    start: 3303,
                                                                                                    end: 3308,
                                                                                                    as_str(): "other",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            TyExpression {
                                                                                                expression: Literal(
                                                                                                    U8(
                                                                                                        3,
                                                                                                    ),
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    31651,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 233,
                                                                                                    end: 236,
                                                                                                    as_str(): "3u8",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    function_decl_id: DeclId(
                                                                                        13319,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 3291,
                                                                                            end: 3357,
                                                                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                                        },
                                                                                    ),
                                                                                    self_state_idx: None,
                                                                                    selector: None,
                                                                                    type_binding: None,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 236,
                                                                                    as_str(): "third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8",
                                                                                },
                                                                            },
                                                                            then: TyExpression {
                                                                                expression: CodeBlock(
                                                                                    TyCodeBlock {
                                                                                        contents: [
                                                                                            TyAstNode {
                                                                                                content: ImplicitReturnExpression(
                                                                                                    TyExpression {
                                                                                                        expression: Literal(
                                                                                                            Boolean(
                                                                                                                false,
                                                                                                            ),
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            71,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 240,
                                                                                                            end: 245,
                                                                                                            as_str(): "false",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 240,
                                                                                                    end: 245,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 240,
                                                                                    end: 245,
                                                                                    as_str(): "false",
                                                                                },
                                                                            },
                                                                            else: Some(
                                                                                TyExpression {
                                                                                    expression: IfExp {
                                                                                        condition: TyExpression {
                                                                                            expression: FunctionApplication {
                                                                                                call_path: CallPath {
                                                                                                    prefixes: [
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "core",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 172,
                                                                                                                end: 254,
                                                                                                                as_str(): "third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 172,
                                                                                                                end: 254,
                                                                                                                as_str(): "third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "eq",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 172,
                                                                                                            end: 254,
                                                                                                            as_str(): "third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8",
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
                                                                                                                src (ptr): 0x00007fe0ae6d93e0,
                                                                                                                path: Some(
                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                ),
                                                                                                                start: 3297,
                                                                                                                end: 3301,
                                                                                                                as_str(): "self",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        TyExpression {
                                                                                                            expression: VariableExpression {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_2",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 172,
                                                                                                                        end: 190,
                                                                                                                        as_str(): "third_match(value)",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 172,
                                                                                                                    end: 190,
                                                                                                                    as_str(): "third_match(value)",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                31651,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 172,
                                                                                                                end: 190,
                                                                                                                as_str(): "third_match(value)",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    (
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ae6d93e0,
                                                                                                                path: Some(
                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                ),
                                                                                                                start: 3303,
                                                                                                                end: 3308,
                                                                                                                as_str(): "other",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        TyExpression {
                                                                                                            expression: Literal(
                                                                                                                U8(
                                                                                                                    4,
                                                                                                                ),
                                                                                                            ),
                                                                                                            return_type: TypeId(
                                                                                                                31651,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 251,
                                                                                                                end: 254,
                                                                                                                as_str(): "4u8",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                                function_decl_id: DeclId(
                                                                                                    13318,
                                                                                                    Span {
                                                                                                        src (ptr): 0x00007fe0ae6d93e0,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 3291,
                                                                                                        end: 3357,
                                                                                                        as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                                                    },
                                                                                                ),
                                                                                                self_state_idx: None,
                                                                                                selector: None,
                                                                                                type_binding: None,
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                71,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 172,
                                                                                                end: 254,
                                                                                                as_str(): "third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8",
                                                                                            },
                                                                                        },
                                                                                        then: TyExpression {
                                                                                            expression: CodeBlock(
                                                                                                TyCodeBlock {
                                                                                                    contents: [
                                                                                                        TyAstNode {
                                                                                                            content: ImplicitReturnExpression(
                                                                                                                TyExpression {
                                                                                                                    expression: Literal(
                                                                                                                        Boolean(
                                                                                                                            false,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    return_type: TypeId(
                                                                                                                        71,
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 258,
                                                                                                                        end: 263,
                                                                                                                        as_str(): "false",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 258,
                                                                                                                end: 263,
                                                                                                                as_str(): "false",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            return_type: TypeId(
                                                                                                71,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 258,
                                                                                                end: 263,
                                                                                                as_str(): "false",
                                                                                            },
                                                                                        },
                                                                                        else: Some(
                                                                                            TyExpression {
                                                                                                expression: IfExp {
                                                                                                    condition: TyExpression {
                                                                                                        expression: FunctionApplication {
                                                                                                            call_path: CallPath {
                                                                                                                prefixes: [
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "core",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 172,
                                                                                                                            end: 272,
                                                                                                                            as_str(): "third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "ops",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 172,
                                                                                                                            end: 272,
                                                                                                                            as_str(): "third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ],
                                                                                                                suffix: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "eq",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 172,
                                                                                                                        end: 272,
                                                                                                                        as_str(): "third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8",
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
                                                                                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                                                                                            path: Some(
                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                            ),
                                                                                                                            start: 3297,
                                                                                                                            end: 3301,
                                                                                                                            as_str(): "self",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    TyExpression {
                                                                                                                        expression: VariableExpression {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: Some(
                                                                                                                                    "__match_return_var_name_2",
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 172,
                                                                                                                                    end: 190,
                                                                                                                                    as_str(): "third_match(value)",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 172,
                                                                                                                                end: 190,
                                                                                                                                as_str(): "third_match(value)",
                                                                                                                            },
                                                                                                                            mutability: Immutable,
                                                                                                                        },
                                                                                                                        return_type: TypeId(
                                                                                                                            31651,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 172,
                                                                                                                            end: 190,
                                                                                                                            as_str(): "third_match(value)",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                (
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                                                                                            path: Some(
                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                            ),
                                                                                                                            start: 3303,
                                                                                                                            end: 3308,
                                                                                                                            as_str(): "other",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    TyExpression {
                                                                                                                        expression: Literal(
                                                                                                                            U8(
                                                                                                                                5,
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        return_type: TypeId(
                                                                                                                            31651,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 269,
                                                                                                                            end: 272,
                                                                                                                            as_str(): "5u8",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ],
                                                                                                            function_decl_id: DeclId(
                                                                                                                13317,
                                                                                                                Span {
                                                                                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                    ),
                                                                                                                    start: 3291,
                                                                                                                    end: 3357,
                                                                                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                                                                },
                                                                                                            ),
                                                                                                            self_state_idx: None,
                                                                                                            selector: None,
                                                                                                            type_binding: None,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            71,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 172,
                                                                                                            end: 272,
                                                                                                            as_str(): "third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8",
                                                                                                        },
                                                                                                    },
                                                                                                    then: TyExpression {
                                                                                                        expression: CodeBlock(
                                                                                                            TyCodeBlock {
                                                                                                                contents: [
                                                                                                                    TyAstNode {
                                                                                                                        content: ImplicitReturnExpression(
                                                                                                                            TyExpression {
                                                                                                                                expression: Literal(
                                                                                                                                    Boolean(
                                                                                                                                        true,
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                                return_type: TypeId(
                                                                                                                                    71,
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 276,
                                                                                                                                    end: 280,
                                                                                                                                    as_str(): "true",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 276,
                                                                                                                            end: 280,
                                                                                                                            as_str(): "true",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ],
                                                                                                            },
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            71,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 276,
                                                                                                            end: 280,
                                                                                                            as_str(): "true",
                                                                                                        },
                                                                                                    },
                                                                                                    else: Some(
                                                                                                        TyExpression {
                                                                                                            expression: CodeBlock(
                                                                                                                TyCodeBlock {
                                                                                                                    contents: [
                                                                                                                        TyAstNode {
                                                                                                                            content: ImplicitReturnExpression(
                                                                                                                                TyExpression {
                                                                                                                                    expression: Literal(
                                                                                                                                        Boolean(
                                                                                                                                            false,
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                    return_type: TypeId(
                                                                                                                                        71,
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 291,
                                                                                                                                        end: 296,
                                                                                                                                        as_str(): "false",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 291,
                                                                                                                                end: 296,
                                                                                                                                as_str(): "false",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            return_type: TypeId(
                                                                                                                71,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 291,
                                                                                                                end: 296,
                                                                                                                as_str(): "false",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    71,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 276,
                                                                                                    end: 280,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        71,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 258,
                                                                                        end: 263,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 240,
                                                                            end: 245,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                71,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 222,
                                                                end: 227,
                                                                as_str(): "false",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 204,
                                                    end: 209,
                                                    as_str(): "false",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 166,
                                            end: 301,
                                            as_str(): "match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 166,
                            end: 301,
                            as_str(): "match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 166,
                    end: 301,
                    as_str(): "match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 144,
                    end: 149,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                31649,
            ),
            initial_type_id: TypeId(
                31650,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09f0b11e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                ),
                start: 151,
                end: 152,
                as_str(): "B",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 125,
        end: 303,
        as_str(): "fn second_match<B>(value: B) -> bool {\n  match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
    type_parameters: [
        B: TypeId(31649),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 157,
        end: 161,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 308,
            end: 319,
            as_str(): "first_match",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
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
                                                        name_override_opt: Some(
                                                            "__match_return_var_name_3",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 350,
                                                            end: 369,
                                                            as_str(): "second_match(value)",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 350,
                                                                        end: 362,
                                                                        as_str(): "second_match",
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 144,
                                                                            end: 149,
                                                                            as_str(): "value",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 323,
                                                                                    end: 328,
                                                                                    as_str(): "value",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 363,
                                                                                end: 368,
                                                                                as_str(): "value",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31664,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 363,
                                                                            end: 368,
                                                                            as_str(): "value",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13330,
                                                                Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 125,
                                                                    end: 303,
                                                                    as_str(): "fn second_match<B>(value: B) -> bool {\n  match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 350,
                                                                        end: 362,
                                                                        as_str(): "second_match",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 350,
                                                            end: 369,
                                                            as_str(): "second_match(value)",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        71,
                                                    ),
                                                    type_ascription: TypeId(
                                                        31666,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 344,
                                            end: 412,
                                            as_str(): "match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }",
                                        },
                                    },
                                    TyAstNode {
                                        content: ImplicitReturnExpression(
                                            TyExpression {
                                                expression: IfExp {
                                                    condition: TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 350,
                                                                            end: 381,
                                                                            as_str(): "second_match(value) {\n    false",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 350,
                                                                            end: 381,
                                                                            as_str(): "second_match(value) {\n    false",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "eq",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 350,
                                                                        end: 381,
                                                                        as_str(): "second_match(value) {\n    false",
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
                                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2930,
                                                                            end: 2934,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "__match_return_var_name_3",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 350,
                                                                                    end: 369,
                                                                                    as_str(): "second_match(value)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 350,
                                                                                end: 369,
                                                                                as_str(): "second_match(value)",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 350,
                                                                            end: 369,
                                                                            as_str(): "second_match(value)",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2936,
                                                                            end: 2941,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 376,
                                                                            end: 381,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13332,
                                                                Span {
                                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2924,
                                                                    end: 2990,
                                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: None,
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 350,
                                                            end: 381,
                                                            as_str(): "second_match(value) {\n    false",
                                                        },
                                                    },
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: ImplicitReturnExpression(
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 385,
                                                                                    end: 389,
                                                                                    as_str(): "2u64",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 385,
                                                                            end: 389,
                                                                            as_str(): "2u64",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 385,
                                                            end: 389,
                                                            as_str(): "2u64",
                                                        },
                                                    },
                                                    else: Some(
                                                        TyExpression {
                                                            expression: IfExp {
                                                                condition: TyExpression {
                                                                    expression: FunctionApplication {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 350,
                                                                                        end: 399,
                                                                                        as_str(): "second_match(value) {\n    false => 2u64,\n    true",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 350,
                                                                                        end: 399,
                                                                                        as_str(): "second_match(value) {\n    false => 2u64,\n    true",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "eq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 350,
                                                                                    end: 399,
                                                                                    as_str(): "second_match(value) {\n    false => 2u64,\n    true",
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
                                                                                        src (ptr): 0x00007fe0ae6d93e0,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                        ),
                                                                                        start: 2930,
                                                                                        end: 2934,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "__match_return_var_name_3",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 350,
                                                                                                end: 369,
                                                                                                as_str(): "second_match(value)",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 350,
                                                                                            end: 369,
                                                                                            as_str(): "second_match(value)",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        71,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 350,
                                                                                        end: 369,
                                                                                        as_str(): "second_match(value)",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ae6d93e0,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                        ),
                                                                                        start: 2936,
                                                                                        end: 2941,
                                                                                        as_str(): "other",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                TyExpression {
                                                                                    expression: Literal(
                                                                                        Boolean(
                                                                                            true,
                                                                                        ),
                                                                                    ),
                                                                                    return_type: TypeId(
                                                                                        71,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 395,
                                                                                        end: 399,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        function_decl_id: DeclId(
                                                                            13331,
                                                                            Span {
                                                                                src (ptr): 0x00007fe0ae6d93e0,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 2924,
                                                                                end: 2990,
                                                                                as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                            },
                                                                        ),
                                                                        self_state_idx: None,
                                                                        selector: None,
                                                                        type_binding: None,
                                                                    },
                                                                    return_type: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 350,
                                                                        end: 399,
                                                                        as_str(): "second_match(value) {\n    false => 2u64,\n    true",
                                                                    },
                                                                },
                                                                then: TyExpression {
                                                                    expression: CodeBlock(
                                                                        TyCodeBlock {
                                                                            contents: [
                                                                                TyAstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        TyExpression {
                                                                                            expression: Literal(
                                                                                                U64(
                                                                                                    3,
                                                                                                ),
                                                                                            ),
                                                                                            return_type: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 403,
                                                                                                end: 407,
                                                                                                as_str(): "3u64",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 403,
                                                                                        end: 407,
                                                                                        as_str(): "3u64",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 403,
                                                                        end: 407,
                                                                        as_str(): "3u64",
                                                                    },
                                                                },
                                                                else: Some(
                                                                    TyExpression {
                                                                        expression: CodeBlock(
                                                                            TyCodeBlock {
                                                                                contents: [
                                                                                    TyAstNode {
                                                                                        content: ImplicitReturnExpression(
                                                                                            TyExpression {
                                                                                                expression: Literal(
                                                                                                    U64(
                                                                                                        3,
                                                                                                    ),
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 403,
                                                                                                    end: 407,
                                                                                                    as_str(): "3u64",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 403,
                                                                                            end: 407,
                                                                                            as_str(): "3u64",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 403,
                                                                            end: 407,
                                                                            as_str(): "3u64",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 403,
                                                                end: 407,
                                                                as_str(): "3u64",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 385,
                                                    end: 389,
                                                    as_str(): "2u64",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 344,
                                            end: 412,
                                            as_str(): "match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 344,
                            end: 412,
                            as_str(): "match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 344,
                    end: 412,
                    as_str(): "match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 323,
                    end: 328,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                31664,
            ),
            initial_type_id: TypeId(
                31665,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09f0b11e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                ),
                start: 330,
                end: 331,
                as_str(): "C",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 305,
        end: 414,
        as_str(): "fn first_match<C>(value: C) -> u64 {\n  match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [
        C: TypeId(31664),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 336,
        end: 339,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 419,
            end: 427,
            as_str(): "third_if",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        true,
                                    ),
                                ),
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 454,
                                    end: 458,
                                    as_str(): "true",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: Literal(
                                                            U8(
                                                                5,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 465,
                                                            end: 468,
                                                            as_str(): "5u8",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 465,
                                                    end: 468,
                                                    as_str(): "5u8",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    50,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 459,
                                    end: 472,
                                    as_str(): "{\n    5u8\n  }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: Expression(
                                                        TyExpression {
                                                            expression: FunctionApplication {
                                                                call_path: CallPath {
                                                                    prefixes: [],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 484,
                                                                            end: 490,
                                                                            as_str(): "revert",
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
                                                                                src (ptr): 0x00007fe0ad8d7980,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                                                                ),
                                                                                start: 582,
                                                                                end: 586,
                                                                                as_str(): "code",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        TyExpression {
                                                                            expression: Literal(
                                                                                U64(
                                                                                    0,
                                                                                ),
                                                                            ),
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 491,
                                                                                end: 492,
                                                                                as_str(): "0",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                function_decl_id: DeclId(
                                                                    13335,
                                                                    Span {
                                                                        src (ptr): 0x00007fe0ad8d7980,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                                                        ),
                                                                        start: 568,
                                                                        end: 615,
                                                                        as_str(): "pub fn revert(code: u64) {\n    __revert(code)\n}",
                                                                    },
                                                                ),
                                                                self_state_idx: None,
                                                                selector: None,
                                                                type_binding: Some(
                                                                    TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 484,
                                                                            end: 490,
                                                                            as_str(): "revert",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                31680,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 484,
                                                                end: 493,
                                                                as_str(): "revert(0)",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 484,
                                                        end: 493,
                                                        as_str(): "revert(0)",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        7215,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 478,
                                        end: 498,
                                        as_str(): "{\n    revert(0);\n  }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            50,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 451,
                            end: 498,
                            as_str(): "if true {\n    5u8\n  } else {\n    revert(0);\n  }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 451,
                    end: 498,
                    as_str(): "if true {\n    5u8\n  } else {\n    revert(0);\n  }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 431,
                    end: 436,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                31673,
            ),
            initial_type_id: TypeId(
                31674,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09f0b11e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                ),
                start: 438,
                end: 439,
                as_str(): "D",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 416,
        end: 500,
        as_str(): "fn third_if<D>(value: D) -> u8 {\n  if true {\n    5u8\n  } else {\n    revert(0);\n  }\n}",
    },
    attributes: {},
    return_type: TypeId(
        50,
    ),
    initial_return_type: TypeId(
        50,
    ),
    type_parameters: [
        D: TypeId(31673),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 444,
        end: 446,
        as_str(): "u8",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 505,
            end: 514,
            as_str(): "second_if",
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
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 544,
                                    end: 549,
                                    as_str(): "third",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 552,
                                                end: 560,
                                                as_str(): "third_if",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 431,
                                                    end: 436,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 518,
                                                            end: 523,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 561,
                                                        end: 566,
                                                        as_str(): "value",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    31682,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 561,
                                                    end: 566,
                                                    as_str(): "value",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13339,
                                        Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 416,
                                            end: 500,
                                            as_str(): "fn third_if<D>(value: D) -> u8 {\n  if true {\n    5u8\n  } else {\n    revert(0);\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 552,
                                                end: 560,
                                                as_str(): "third_if",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    50,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 552,
                                    end: 567,
                                    as_str(): "third_if(value)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31684,
                            ),
                            type_ascription: TypeId(
                                31684,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 540,
                    end: 568,
                    as_str(): "let third = third_if(value);",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: LazyOperator {
                                    op: Or,
                                    lhs: TyExpression {
                                        expression: LazyOperator {
                                            op: Or,
                                            lhs: TyExpression {
                                                expression: LazyOperator {
                                                    op: Or,
                                                    lhs: TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 580,
                                                                            end: 582,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 580,
                                                                            end: 582,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 580,
                                                                        end: 582,
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
                                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3297,
                                                                            end: 3301,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 544,
                                                                                    end: 549,
                                                                                    as_str(): "third",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 574,
                                                                                end: 579,
                                                                                as_str(): "third",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31684,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 574,
                                                                            end: 579,
                                                                            as_str(): "third",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3303,
                                                                            end: 3308,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U8(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 583,
                                                                            end: 586,
                                                                            as_str(): "1u8",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13340,
                                                                Span {
                                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 3291,
                                                                    end: 3357,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 580,
                                                                        end: 582,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 574,
                                                            end: 586,
                                                            as_str(): "third == 1u8",
                                                        },
                                                    },
                                                    rhs: TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 596,
                                                                            end: 598,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 596,
                                                                            end: 598,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 596,
                                                                        end: 598,
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
                                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3297,
                                                                            end: 3301,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 544,
                                                                                    end: 549,
                                                                                    as_str(): "third",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 590,
                                                                                end: 595,
                                                                                as_str(): "third",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31684,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 590,
                                                                            end: 595,
                                                                            as_str(): "third",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3303,
                                                                            end: 3308,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U8(
                                                                                2,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 599,
                                                                            end: 602,
                                                                            as_str(): "2u8",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13341,
                                                                Span {
                                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 3291,
                                                                    end: 3357,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 596,
                                                                        end: 598,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 590,
                                                            end: 602,
                                                            as_str(): "third == 2u8",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 574,
                                                    end: 602,
                                                    as_str(): "third == 1u8 || third == 2u8",
                                                },
                                            },
                                            rhs: TyExpression {
                                                expression: FunctionApplication {
                                                    call_path: CallPath {
                                                        prefixes: [
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "core",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 612,
                                                                    end: 614,
                                                                    as_str(): "==",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 612,
                                                                    end: 614,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 612,
                                                                end: 614,
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
                                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 3297,
                                                                    end: 3301,
                                                                    as_str(): "self",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 544,
                                                                            end: 549,
                                                                            as_str(): "third",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 606,
                                                                        end: 611,
                                                                        as_str(): "third",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31684,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 606,
                                                                    end: 611,
                                                                    as_str(): "third",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 3303,
                                                                    end: 3308,
                                                                    as_str(): "other",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U8(
                                                                        3,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    50,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 615,
                                                                    end: 618,
                                                                    as_str(): "3u8",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13342,
                                                        Span {
                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3291,
                                                            end: 3357,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 612,
                                                                end: 614,
                                                                as_str(): "==",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 606,
                                                    end: 618,
                                                    as_str(): "third == 3u8",
                                                },
                                            },
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 574,
                                            end: 618,
                                            as_str(): "third == 1u8 || third == 2u8 || third == 3u8",
                                        },
                                    },
                                    rhs: TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 628,
                                                            end: 630,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 628,
                                                            end: 630,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 628,
                                                        end: 630,
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
                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3297,
                                                            end: 3301,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 544,
                                                                    end: 549,
                                                                    as_str(): "third",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 622,
                                                                end: 627,
                                                                as_str(): "third",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31684,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 622,
                                                            end: 627,
                                                            as_str(): "third",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3303,
                                                            end: 3308,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U8(
                                                                4,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 631,
                                                            end: 634,
                                                            as_str(): "4u8",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13343,
                                                Span {
                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3291,
                                                    end: 3357,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 628,
                                                        end: 630,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 622,
                                            end: 634,
                                            as_str(): "third == 4u8",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 574,
                                    end: 634,
                                    as_str(): "third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: Literal(
                                                            Boolean(
                                                                false,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 641,
                                                            end: 646,
                                                            as_str(): "false",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 641,
                                                    end: 646,
                                                    as_str(): "false",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 635,
                                    end: 650,
                                    as_str(): "{\n    false\n  }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: IfExp {
                                        condition: TyExpression {
                                            expression: FunctionApplication {
                                                call_path: CallPath {
                                                    prefixes: [
                                                        BaseIdent {
                                                            name_override_opt: Some(
                                                                "core",
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 665,
                                                                end: 667,
                                                                as_str(): "==",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        BaseIdent {
                                                            name_override_opt: Some(
                                                                "ops",
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 665,
                                                                end: 667,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 665,
                                                            end: 667,
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
                                                                src (ptr): 0x00007fe0ae6d93e0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                ),
                                                                start: 3297,
                                                                end: 3301,
                                                                as_str(): "self",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        TyExpression {
                                                            expression: VariableExpression {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 544,
                                                                        end: 549,
                                                                        as_str(): "third",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 659,
                                                                    end: 664,
                                                                    as_str(): "third",
                                                                },
                                                                mutability: Immutable,
                                                            },
                                                            return_type: TypeId(
                                                                31684,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 659,
                                                                end: 664,
                                                                as_str(): "third",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ae6d93e0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                ),
                                                                start: 3303,
                                                                end: 3308,
                                                                as_str(): "other",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        TyExpression {
                                                            expression: Literal(
                                                                U8(
                                                                    5,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                50,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 668,
                                                                end: 671,
                                                                as_str(): "5u8",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                function_decl_id: DeclId(
                                                    13344,
                                                    Span {
                                                        src (ptr): 0x00007fe0ae6d93e0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                        ),
                                                        start: 3291,
                                                        end: 3357,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 665,
                                                            end: 667,
                                                            as_str(): "==",
                                                        },
                                                    },
                                                ),
                                            },
                                            return_type: TypeId(
                                                71,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 659,
                                                end: 671,
                                                as_str(): "third == 5u8",
                                            },
                                        },
                                        then: TyExpression {
                                            expression: CodeBlock(
                                                TyCodeBlock {
                                                    contents: [
                                                        TyAstNode {
                                                            content: ImplicitReturnExpression(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        Boolean(
                                                                            true,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 678,
                                                                        end: 682,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 678,
                                                                end: 682,
                                                                as_str(): "true",
                                                            },
                                                        },
                                                    ],
                                                },
                                            ),
                                            return_type: TypeId(
                                                71,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 672,
                                                end: 686,
                                                as_str(): "{\n    true\n  }",
                                            },
                                        },
                                        else: Some(
                                            TyExpression {
                                                expression: CodeBlock(
                                                    TyCodeBlock {
                                                        contents: [
                                                            TyAstNode {
                                                                content: ImplicitReturnExpression(
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 698,
                                                                            end: 703,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 698,
                                                                    end: 703,
                                                                    as_str(): "false",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 692,
                                                    end: 707,
                                                    as_str(): "{\n    false\n  }",
                                                },
                                            },
                                        ),
                                    },
                                    return_type: TypeId(
                                        71,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 656,
                                        end: 707,
                                        as_str(): "if third == 5u8 {\n    true\n  } else {\n    false\n  }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 571,
                            end: 707,
                            as_str(): "if third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8 {\n    false\n  } else if third == 5u8 {\n    true\n  } else {\n    false\n  }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 571,
                    end: 707,
                    as_str(): "if third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8 {\n    false\n  } else if third == 5u8 {\n    true\n  } else {\n    false\n  }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 518,
                    end: 523,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                31682,
            ),
            initial_type_id: TypeId(
                31683,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09f0b11e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                ),
                start: 525,
                end: 526,
                as_str(): "E",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 502,
        end: 709,
        as_str(): "fn second_if<E>(value: E) -> bool {\n  let third = third_if(value);\n  if third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8 {\n    false\n  } else if third == 5u8 {\n    true\n  } else {\n    false\n  }\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
    type_parameters: [
        E: TypeId(31682),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 531,
        end: 535,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 714,
            end: 722,
            as_str(): "first_if",
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
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 751,
                                    end: 757,
                                    as_str(): "second",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 760,
                                                end: 769,
                                                as_str(): "second_if",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 518,
                                                    end: 523,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 726,
                                                            end: 731,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 770,
                                                        end: 775,
                                                        as_str(): "value",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    31702,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 770,
                                                    end: 775,
                                                    as_str(): "value",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13354,
                                        Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 502,
                                            end: 709,
                                            as_str(): "fn second_if<E>(value: E) -> bool {\n  let third = third_if(value);\n  if third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8 {\n    false\n  } else if third == 5u8 {\n    true\n  } else {\n    false\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 760,
                                                end: 769,
                                                as_str(): "second_if",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 760,
                                    end: 776,
                                    as_str(): "second_if(value)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                31704,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 747,
                    end: 777,
                    as_str(): "let second = second_if(value);",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 790,
                                                    end: 792,
                                                    as_str(): "==",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 790,
                                                    end: 792,
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 790,
                                                end: 792,
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
                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2930,
                                                    end: 2934,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 751,
                                                            end: 757,
                                                            as_str(): "second",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 783,
                                                        end: 789,
                                                        as_str(): "second",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 783,
                                                    end: 789,
                                                    as_str(): "second",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2936,
                                                    end: 2941,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 793,
                                                    end: 798,
                                                    as_str(): "false",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13355,
                                        Span {
                                            src (ptr): 0x00007fe0ae6d93e0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 2924,
                                            end: 2990,
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 790,
                                                end: 792,
                                                as_str(): "==",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 783,
                                    end: 798,
                                    as_str(): "second == false",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 805,
                                                            end: 809,
                                                            as_str(): "2u64",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 805,
                                                    end: 809,
                                                    as_str(): "2u64",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 799,
                                    end: 813,
                                    as_str(): "{\n    2u64\n  }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    3,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 825,
                                                                end: 829,
                                                                as_str(): "3u64",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 825,
                                                        end: 829,
                                                        as_str(): "3u64",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 819,
                                        end: 833,
                                        as_str(): "{\n    3u64\n  }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 780,
                            end: 833,
                            as_str(): "if second == false {\n    2u64\n  } else {\n    3u64\n  }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 780,
                    end: 833,
                    as_str(): "if second == false {\n    2u64\n  } else {\n    3u64\n  }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 726,
                    end: 731,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                31702,
            ),
            initial_type_id: TypeId(
                31703,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09f0b11e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                ),
                start: 733,
                end: 734,
                as_str(): "F",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 711,
        end: 835,
        as_str(): "fn first_if<F>(value: F) -> u64 {\n  let second = second_if(value);\n  if second == false {\n    2u64\n  } else {\n    3u64\n  }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [
        F: TypeId(31702),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 739,
        end: 742,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 840,
            end: 853,
            as_str(): "double_double",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 870,
                                    end: 876,
                                    as_str(): "second",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 890,
                                end: 896,
                                as_str(): "second",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            31713,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 890,
                            end: 896,
                            as_str(): "second",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 890,
                    end: 896,
                    as_str(): "second",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 860,
                    end: 865,
                    as_str(): "first",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                31712,
            ),
            initial_type_id: TypeId(
                31714,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09f0b11e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                ),
                start: 867,
                end: 868,
                as_str(): "Y",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 870,
                    end: 876,
                    as_str(): "second",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                31713,
            ),
            initial_type_id: TypeId(
                31715,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09f0b11e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                ),
                start: 878,
                end: 879,
                as_str(): "Z",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 837,
        end: 898,
        as_str(): "fn double_double<Y, Z>(first: Y, second: Z) -> Z {\n  second\n}",
    },
    attributes: {},
    return_type: TypeId(
        31713,
    ),
    initial_return_type: TypeId(
        31716,
    ),
    type_parameters: [
        Y: TypeId(31712),
        Z: TypeId(31713),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 884,
        end: 885,
        as_str(): "Z",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 903,
            end: 909,
            as_str(): "double",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 937,
                                        end: 950,
                                        as_str(): "double_double",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 860,
                                            end: 865,
                                            as_str(): "first",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Literal(
                                            Boolean(
                                                false,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 951,
                                            end: 956,
                                            as_str(): "false",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 870,
                                            end: 876,
                                            as_str(): "second",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 913,
                                                    end: 923,
                                                    as_str(): "the_second",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 958,
                                                end: 968,
                                                as_str(): "the_second",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            31718,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 958,
                                            end: 968,
                                            as_str(): "the_second",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13359,
                                Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 837,
                                    end: 898,
                                    as_str(): "fn double_double<Y, Z>(first: Y, second: Z) -> Z {\n  second\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 937,
                                        end: 950,
                                        as_str(): "double_double",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31722,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 937,
                            end: 969,
                            as_str(): "double_double(false, the_second)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 937,
                    end: 969,
                    as_str(): "double_double(false, the_second)",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 913,
                    end: 923,
                    as_str(): "the_second",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                31718,
            ),
            initial_type_id: TypeId(
                31719,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09f0b11e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                ),
                start: 925,
                end: 926,
                as_str(): "Y",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 900,
        end: 971,
        as_str(): "fn double<Y>(the_second: Y) -> Y {\n  double_double(false, the_second)\n}",
    },
    attributes: {},
    return_type: TypeId(
        31718,
    ),
    initial_return_type: TypeId(
        31720,
    ),
    type_parameters: [
        Y: TypeId(31718),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 931,
        end: 932,
        as_str(): "Y",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 976,
            end: 989,
            as_str(): "generic_match",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
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
                                                        name_override_opt: Some(
                                                            "__match_return_var_name_4",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1020,
                                                            end: 1025,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 993,
                                                                    end: 998,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1020,
                                                                end: 1025,
                                                                as_str(): "value",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31726,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1020,
                                                            end: 1025,
                                                            as_str(): "value",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        31726,
                                                    ),
                                                    type_ascription: TypeId(
                                                        31728,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1014,
                                            end: 1048,
                                            as_str(): "match value {\n    foo => 3u64,\n  }",
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1032,
                                                                                    end: 1035,
                                                                                    as_str(): "foo",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            body: TyExpression {
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "__match_return_var_name_4",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 1020,
                                                                                            end: 1025,
                                                                                            as_str(): "value",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1020,
                                                                                        end: 1025,
                                                                                        as_str(): "value",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    31726,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1020,
                                                                                    end: 1025,
                                                                                    as_str(): "value",
                                                                                },
                                                                            },
                                                                            mutability: Immutable,
                                                                            return_type: TypeId(
                                                                                31726,
                                                                            ),
                                                                            type_ascription: TypeId(
                                                                                31726,
                                                                            ),
                                                                            type_ascription_span: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1032,
                                                                    end: 1035,
                                                                    as_str(): "foo",
                                                                },
                                                            },
                                                            TyAstNode {
                                                                content: ImplicitReturnExpression(
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1039,
                                                                            end: 1043,
                                                                            as_str(): "3u64",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1039,
                                                                    end: 1043,
                                                                    as_str(): "3u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1039,
                                                    end: 1043,
                                                    as_str(): "3u64",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1014,
                                            end: 1048,
                                            as_str(): "match value {\n    foo => 3u64,\n  }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1014,
                            end: 1048,
                            as_str(): "match value {\n    foo => 3u64,\n  }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1014,
                    end: 1048,
                    as_str(): "match value {\n    foo => 3u64,\n  }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 993,
                    end: 998,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                31726,
            ),
            initial_type_id: TypeId(
                31727,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09f0b11e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                ),
                start: 1000,
                end: 1001,
                as_str(): "G",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 973,
        end: 1050,
        as_str(): "fn generic_match<G>(value: G) -> u64 {\n  match value {\n    foo => 3u64,\n  }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [
        G: TypeId(31726),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 1006,
        end: 1009,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 1055,
            end: 1065,
            as_str(): "generic_if",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        true,
                                    ),
                                ),
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1093,
                                    end: 1097,
                                    as_str(): "true",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                3,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1104,
                                                            end: 1108,
                                                            as_str(): "3u64",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1104,
                                                    end: 1108,
                                                    as_str(): "3u64",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1098,
                                    end: 1112,
                                    as_str(): "{\n    3u64\n  }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    1,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1124,
                                                                end: 1128,
                                                                as_str(): "1u64",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1124,
                                                        end: 1128,
                                                        as_str(): "1u64",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1118,
                                        end: 1132,
                                        as_str(): "{\n    1u64\n  }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1090,
                            end: 1132,
                            as_str(): "if true {\n    3u64\n  } else {\n    1u64\n  }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1090,
                    end: 1132,
                    as_str(): "if true {\n    3u64\n  } else {\n    1u64\n  }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1069,
                    end: 1074,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                31733,
            ),
            initial_type_id: TypeId(
                31734,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09f0b11e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                ),
                start: 1076,
                end: 1077,
                as_str(): "H",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 1052,
        end: 1134,
        as_str(): "fn generic_if<H>(value: H) -> u64 {\n  if true {\n    3u64\n  } else {\n    1u64\n  }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [
        H: TypeId(31733),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 1082,
        end: 1085,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 1139,
            end: 1143,
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
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1161,
                                    end: 1162,
                                    as_str(): "a",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1165,
                                                end: 1176,
                                                as_str(): "first_match",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 323,
                                                    end: 328,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1177,
                                                    end: 1181,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13373,
                                        Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 305,
                                            end: 414,
                                            as_str(): "fn first_match<C>(value: C) -> u64 {\n  match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1165,
                                                end: 1176,
                                                as_str(): "first_match",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1165,
                                    end: 1182,
                                    as_str(): "first_match(true)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31738,
                            ),
                            type_ascription: TypeId(
                                31738,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1157,
                    end: 1183,
                    as_str(): "let a = first_match(true);",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1186,
                                        end: 1192,
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
                                            src (ptr): 0x00007fe0a7282c80,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1195,
                                                            end: 1197,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1195,
                                                            end: 1197,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1195,
                                                        end: 1197,
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
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1161,
                                                                    end: 1162,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1193,
                                                                end: 1194,
                                                                as_str(): "a",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31738,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1193,
                                                            end: 1194,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                                3,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1198,
                                                            end: 1199,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13375,
                                                Span {
                                                    src (ptr): 0x00007fe0ae6d93e0,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1195,
                                                        end: 1197,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1193,
                                            end: 1199,
                                            as_str(): "a == 3",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13376,
                                Span {
                                    src (ptr): 0x00007fe0a7282c80,
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1186,
                                        end: 1192,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31746,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1186,
                            end: 1200,
                            as_str(): "assert(a == 3)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1186,
                    end: 1200,
                    as_str(): "assert(a == 3)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1209,
                                    end: 1210,
                                    as_str(): "b",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1213,
                                                end: 1224,
                                                as_str(): "first_match",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 323,
                                                    end: 328,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U8(
                                                        1,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    50,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1225,
                                                    end: 1228,
                                                    as_str(): "1u8",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13387,
                                        Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 305,
                                            end: 414,
                                            as_str(): "fn first_match<C>(value: C) -> u64 {\n  match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1213,
                                                end: 1224,
                                                as_str(): "first_match",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1213,
                                    end: 1229,
                                    as_str(): "first_match(1u8)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31747,
                            ),
                            type_ascription: TypeId(
                                31747,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1205,
                    end: 1230,
                    as_str(): "let b = first_match(1u8);",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1233,
                                        end: 1239,
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
                                            src (ptr): 0x00007fe0a7282c80,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1242,
                                                            end: 1244,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1242,
                                                            end: 1244,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1242,
                                                        end: 1244,
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
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1209,
                                                                    end: 1210,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1240,
                                                                end: 1241,
                                                                as_str(): "b",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31747,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1240,
                                                            end: 1241,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                                3,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1245,
                                                            end: 1246,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13389,
                                                Span {
                                                    src (ptr): 0x00007fe0ae6d93e0,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1242,
                                                        end: 1244,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1240,
                                            end: 1246,
                                            as_str(): "b == 3",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13390,
                                Span {
                                    src (ptr): 0x00007fe0a7282c80,
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1233,
                                        end: 1239,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31755,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1233,
                            end: 1247,
                            as_str(): "assert(b == 3)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1233,
                    end: 1247,
                    as_str(): "assert(b == 3)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1256,
                                    end: 1257,
                                    as_str(): "c",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1260,
                                                end: 1268,
                                                as_str(): "first_if",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 726,
                                                    end: 731,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1269,
                                                    end: 1273,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13401,
                                        Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 711,
                                            end: 835,
                                            as_str(): "fn first_if<F>(value: F) -> u64 {\n  let second = second_if(value);\n  if second == false {\n    2u64\n  } else {\n    3u64\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1260,
                                                end: 1268,
                                                as_str(): "first_if",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1260,
                                    end: 1274,
                                    as_str(): "first_if(true)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31756,
                            ),
                            type_ascription: TypeId(
                                31756,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1252,
                    end: 1275,
                    as_str(): "let c = first_if(true);",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1278,
                                        end: 1284,
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
                                            src (ptr): 0x00007fe0a7282c80,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1287,
                                                            end: 1289,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1287,
                                                            end: 1289,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1287,
                                                        end: 1289,
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
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1256,
                                                                    end: 1257,
                                                                    as_str(): "c",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1285,
                                                                end: 1286,
                                                                as_str(): "c",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31756,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1285,
                                                            end: 1286,
                                                            as_str(): "c",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                                3,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1290,
                                                            end: 1291,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13403,
                                                Span {
                                                    src (ptr): 0x00007fe0ae6d93e0,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1287,
                                                        end: 1289,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1285,
                                            end: 1291,
                                            as_str(): "c == 3",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13404,
                                Span {
                                    src (ptr): 0x00007fe0a7282c80,
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1278,
                                        end: 1284,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31764,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1278,
                            end: 1292,
                            as_str(): "assert(c == 3)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1278,
                    end: 1292,
                    as_str(): "assert(c == 3)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1301,
                                    end: 1302,
                                    as_str(): "d",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1305,
                                                end: 1313,
                                                as_str(): "first_if",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 726,
                                                    end: 731,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U8(
                                                        1,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    50,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1314,
                                                    end: 1317,
                                                    as_str(): "1u8",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13415,
                                        Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 711,
                                            end: 835,
                                            as_str(): "fn first_if<F>(value: F) -> u64 {\n  let second = second_if(value);\n  if second == false {\n    2u64\n  } else {\n    3u64\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1305,
                                                end: 1313,
                                                as_str(): "first_if",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1305,
                                    end: 1318,
                                    as_str(): "first_if(1u8)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31765,
                            ),
                            type_ascription: TypeId(
                                31765,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1297,
                    end: 1319,
                    as_str(): "let d = first_if(1u8);",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1322,
                                        end: 1328,
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
                                            src (ptr): 0x00007fe0a7282c80,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1331,
                                                            end: 1333,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1331,
                                                            end: 1333,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1331,
                                                        end: 1333,
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
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1301,
                                                                    end: 1302,
                                                                    as_str(): "d",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1329,
                                                                end: 1330,
                                                                as_str(): "d",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31765,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1329,
                                                            end: 1330,
                                                            as_str(): "d",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                                3,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1334,
                                                            end: 1335,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13417,
                                                Span {
                                                    src (ptr): 0x00007fe0ae6d93e0,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1331,
                                                        end: 1333,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1329,
                                            end: 1335,
                                            as_str(): "d == 3",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13418,
                                Span {
                                    src (ptr): 0x00007fe0a7282c80,
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1322,
                                        end: 1328,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31773,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1322,
                            end: 1336,
                            as_str(): "assert(d == 3)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1322,
                    end: 1336,
                    as_str(): "assert(d == 3)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1345,
                                    end: 1346,
                                    as_str(): "e",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1349,
                                                end: 1362,
                                                as_str(): "generic_match",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 993,
                                                    end: 998,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        6,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1363,
                                                    end: 1364,
                                                    as_str(): "6",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13420,
                                        Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 973,
                                            end: 1050,
                                            as_str(): "fn generic_match<G>(value: G) -> u64 {\n  match value {\n    foo => 3u64,\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1349,
                                                end: 1362,
                                                as_str(): "generic_match",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1349,
                                    end: 1365,
                                    as_str(): "generic_match(6)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31774,
                            ),
                            type_ascription: TypeId(
                                31774,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1341,
                    end: 1366,
                    as_str(): "let e = generic_match(6);",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1369,
                                        end: 1375,
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
                                            src (ptr): 0x00007fe0a7282c80,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1378,
                                                            end: 1380,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1378,
                                                            end: 1380,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1378,
                                                        end: 1380,
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
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1345,
                                                                    end: 1346,
                                                                    as_str(): "e",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1376,
                                                                end: 1377,
                                                                as_str(): "e",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31774,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1376,
                                                            end: 1377,
                                                            as_str(): "e",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                                3,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1381,
                                                            end: 1382,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13422,
                                                Span {
                                                    src (ptr): 0x00007fe0ae6d93e0,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1378,
                                                        end: 1380,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1376,
                                            end: 1382,
                                            as_str(): "e == 3",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13423,
                                Span {
                                    src (ptr): 0x00007fe0a7282c80,
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1369,
                                        end: 1375,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31783,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1369,
                            end: 1383,
                            as_str(): "assert(e == 3)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1369,
                    end: 1383,
                    as_str(): "assert(e == 3)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1392,
                                    end: 1393,
                                    as_str(): "f",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1396,
                                                end: 1409,
                                                as_str(): "generic_match",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 993,
                                                    end: 998,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1410,
                                                    end: 1415,
                                                    as_str(): "false",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13425,
                                        Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 973,
                                            end: 1050,
                                            as_str(): "fn generic_match<G>(value: G) -> u64 {\n  match value {\n    foo => 3u64,\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1396,
                                                end: 1409,
                                                as_str(): "generic_match",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1396,
                                    end: 1416,
                                    as_str(): "generic_match(false)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31784,
                            ),
                            type_ascription: TypeId(
                                31784,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1388,
                    end: 1417,
                    as_str(): "let f = generic_match(false);",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1420,
                                        end: 1426,
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
                                            src (ptr): 0x00007fe0a7282c80,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1429,
                                                            end: 1431,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1429,
                                                            end: 1431,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1429,
                                                        end: 1431,
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
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1392,
                                                                    end: 1393,
                                                                    as_str(): "f",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1427,
                                                                end: 1428,
                                                                as_str(): "f",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31784,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1427,
                                                            end: 1428,
                                                            as_str(): "f",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                                3,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1432,
                                                            end: 1433,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13427,
                                                Span {
                                                    src (ptr): 0x00007fe0ae6d93e0,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1429,
                                                        end: 1431,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1427,
                                            end: 1433,
                                            as_str(): "f == 3",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13428,
                                Span {
                                    src (ptr): 0x00007fe0a7282c80,
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1420,
                                        end: 1426,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31792,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1420,
                            end: 1434,
                            as_str(): "assert(f == 3)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1420,
                    end: 1434,
                    as_str(): "assert(f == 3)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1443,
                                    end: 1444,
                                    as_str(): "g",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1447,
                                                end: 1457,
                                                as_str(): "generic_if",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1069,
                                                    end: 1074,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        6,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1458,
                                                    end: 1459,
                                                    as_str(): "6",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13430,
                                        Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1052,
                                            end: 1134,
                                            as_str(): "fn generic_if<H>(value: H) -> u64 {\n  if true {\n    3u64\n  } else {\n    1u64\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1447,
                                                end: 1457,
                                                as_str(): "generic_if",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1447,
                                    end: 1460,
                                    as_str(): "generic_if(6)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31793,
                            ),
                            type_ascription: TypeId(
                                31793,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1439,
                    end: 1461,
                    as_str(): "let g = generic_if(6);",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1464,
                                        end: 1470,
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
                                            src (ptr): 0x00007fe0a7282c80,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1473,
                                                            end: 1475,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1473,
                                                            end: 1475,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1473,
                                                        end: 1475,
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
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1443,
                                                                    end: 1444,
                                                                    as_str(): "g",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1471,
                                                                end: 1472,
                                                                as_str(): "g",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31793,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1471,
                                                            end: 1472,
                                                            as_str(): "g",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                                3,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1476,
                                                            end: 1477,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13432,
                                                Span {
                                                    src (ptr): 0x00007fe0ae6d93e0,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1473,
                                                        end: 1475,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1471,
                                            end: 1477,
                                            as_str(): "g == 3",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13433,
                                Span {
                                    src (ptr): 0x00007fe0a7282c80,
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1464,
                                        end: 1470,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31802,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1464,
                            end: 1478,
                            as_str(): "assert(g == 3)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1464,
                    end: 1478,
                    as_str(): "assert(g == 3)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1487,
                                    end: 1488,
                                    as_str(): "h",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1491,
                                                end: 1501,
                                                as_str(): "generic_if",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1069,
                                                    end: 1074,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1502,
                                                    end: 1507,
                                                    as_str(): "false",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13435,
                                        Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1052,
                                            end: 1134,
                                            as_str(): "fn generic_if<H>(value: H) -> u64 {\n  if true {\n    3u64\n  } else {\n    1u64\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1491,
                                                end: 1501,
                                                as_str(): "generic_if",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1491,
                                    end: 1508,
                                    as_str(): "generic_if(false)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31803,
                            ),
                            type_ascription: TypeId(
                                31803,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1483,
                    end: 1509,
                    as_str(): "let h = generic_if(false);",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1512,
                                        end: 1518,
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
                                            src (ptr): 0x00007fe0a7282c80,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1521,
                                                            end: 1523,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1521,
                                                            end: 1523,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1521,
                                                        end: 1523,
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
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1487,
                                                                    end: 1488,
                                                                    as_str(): "h",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1519,
                                                                end: 1520,
                                                                as_str(): "h",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31803,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1519,
                                                            end: 1520,
                                                            as_str(): "h",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ae6d93e0,
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
                                                                3,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1524,
                                                            end: 1525,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13437,
                                                Span {
                                                    src (ptr): 0x00007fe0ae6d93e0,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1521,
                                                        end: 1523,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1519,
                                            end: 1525,
                                            as_str(): "h == 3",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13438,
                                Span {
                                    src (ptr): 0x00007fe0a7282c80,
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1512,
                                        end: 1518,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31811,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1512,
                            end: 1526,
                            as_str(): "assert(h == 3)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1512,
                    end: 1526,
                    as_str(): "assert(h == 3)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1535,
                                    end: 1536,
                                    as_str(): "i",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1539,
                                                end: 1545,
                                                as_str(): "double",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 913,
                                                    end: 923,
                                                    as_str(): "the_second",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1546,
                                                    end: 1551,
                                                    as_str(): "10u32",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13444,
                                        Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 900,
                                            end: 971,
                                            as_str(): "fn double<Y>(the_second: Y) -> Y {\n  double_double(false, the_second)\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1539,
                                                end: 1545,
                                                as_str(): "double",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    31813,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1539,
                                    end: 1552,
                                    as_str(): "double(10u32)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31812,
                            ),
                            type_ascription: TypeId(
                                31812,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1531,
                    end: 1553,
                    as_str(): "let i = double(10u32);",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1556,
                                        end: 1562,
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
                                            src (ptr): 0x00007fe0a7282c80,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1565,
                                                            end: 1567,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1565,
                                                            end: 1567,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1565,
                                                        end: 1567,
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
                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3114,
                                                            end: 3118,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1535,
                                                                    end: 1536,
                                                                    as_str(): "i",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1563,
                                                                end: 1564,
                                                                as_str(): "i",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31812,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1563,
                                                            end: 1564,
                                                            as_str(): "i",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ae6d93e0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3120,
                                                            end: 3125,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U32(
                                                                10,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1568,
                                                            end: 1573,
                                                            as_str(): "10u32",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13446,
                                                Span {
                                                    src (ptr): 0x00007fe0ae6d93e0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3108,
                                                    end: 3174,
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1565,
                                                        end: 1567,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1563,
                                            end: 1573,
                                            as_str(): "i == 10u32",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13447,
                                Span {
                                    src (ptr): 0x00007fe0a7282c80,
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1556,
                                        end: 1562,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31820,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1556,
                            end: 1574,
                            as_str(): "assert(i == 10u32)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1556,
                    end: 1574,
                    as_str(): "assert(i == 10u32)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1583,
                                    end: 1584,
                                    as_str(): "j",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1587,
                                                end: 1593,
                                                as_str(): "double",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 913,
                                                    end: 923,
                                                    as_str(): "the_second",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1594,
                                                    end: 1598,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13450,
                                        Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 900,
                                            end: 971,
                                            as_str(): "fn double<Y>(the_second: Y) -> Y {\n  double_double(false, the_second)\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1587,
                                                end: 1593,
                                                as_str(): "double",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    31822,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1587,
                                    end: 1599,
                                    as_str(): "double(true)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31822,
                            ),
                            type_ascription: TypeId(
                                31821,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1579,
                    end: 1600,
                    as_str(): "let j = double(true);",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1603,
                                        end: 1609,
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
                                            src (ptr): 0x00007fe0a7282c80,
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
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1583,
                                                    end: 1584,
                                                    as_str(): "j",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 1610,
                                                end: 1611,
                                                as_str(): "j",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            31822,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1610,
                                            end: 1611,
                                            as_str(): "j",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13452,
                                Span {
                                    src (ptr): 0x00007fe0a7282c80,
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1603,
                                        end: 1609,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31826,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1603,
                            end: 1612,
                            as_str(): "assert(j)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1603,
                    end: 1612,
                    as_str(): "assert(j)",
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
                            31827,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1617,
                            end: 1618,
                            as_str(): "1",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09f0b11e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                    ),
                    start: 1617,
                    end: 1618,
                    as_str(): "1",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 1136,
        end: 1620,
        as_str(): "fn main() -> u64 {\n  let a = first_match(true);\n  assert(a == 3);\n\n  let b = first_match(1u8);\n  assert(b == 3);\n\n  let c = first_if(true);\n  assert(c == 3);\n\n  let d = first_if(1u8);\n  assert(d == 3);\n\n  let e = generic_match(6);\n  assert(e == 3);\n\n  let f = generic_match(false);\n  assert(f == 3);\n\n  let g = generic_if(6);\n  assert(g == 3);\n\n  let h = generic_if(false);\n  assert(h == 3);\n\n  let i = double(10u32);\n  assert(i == 10u32);\n\n  let j = double(true);\n  assert(j);\n\n  1\n}",
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
        src (ptr): 0x00007fe09f0b11e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
        ),
        start: 1149,
        end: 1152,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

