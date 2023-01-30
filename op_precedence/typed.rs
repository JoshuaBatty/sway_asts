TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe07c4ea020,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
            ),
            start: 80,
            end: 84,
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
                                    src (ptr): 0x00007fe07c4ea020,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                    ),
                                    start: 105,
                                    end: 106,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: LazyOperator {
                                    op: And,
                                    lhs: TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c4ea020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                            ),
                                                            start: 111,
                                                            end: 112,
                                                            as_str(): "<",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c4ea020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                            ),
                                                            start: 111,
                                                            end: 112,
                                                            as_str(): "<",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "lt",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe07c4ea020,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                        ),
                                                        start: 111,
                                                        end: 112,
                                                        as_str(): "<",
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
                                                            src (ptr): 0x00007fe070595ec0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 4000,
                                                            end: 4004,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                4,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c4ea020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                            ),
                                                            start: 109,
                                                            end: 110,
                                                            as_str(): "4",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe070595ec0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 4006,
                                                            end: 4011,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                5,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c4ea020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                            ),
                                                            start: 113,
                                                            end: 114,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                544,
                                                Span {
                                                    src (ptr): 0x00007fe070595ec0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3994,
                                                    end: 4129,
                                                    as_str(): "fn lt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe07c4ea020,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                        ),
                                                        start: 111,
                                                        end: 112,
                                                        as_str(): "<",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c4ea020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                            ),
                                            start: 109,
                                            end: 114,
                                            as_str(): "4 < 5",
                                        },
                                    },
                                    rhs: TyExpression {
                                        expression: Literal(
                                            Boolean(
                                                false,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c4ea020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                            ),
                                            start: 118,
                                            end: 123,
                                            as_str(): "false",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c4ea020,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                    ),
                                    start: 109,
                                    end: 123,
                                    as_str(): "4 < 5 && false",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                7252,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe07c4ea020,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                    ),
                    start: 101,
                    end: 124,
                    as_str(): "let a = 4 < 5 && false;",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe07c4ea020,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                    ),
                                    start: 105,
                                    end: 106,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe07c4ea020,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                ),
                                start: 129,
                                end: 130,
                                as_str(): "a",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe07c4ea020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                            ),
                            start: 129,
                            end: 130,
                            as_str(): "a",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe07c4ea020,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                    ),
                    start: 129,
                    end: 130,
                    as_str(): "a",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe07c4ea020,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
        ),
        start: 77,
        end: 132,
        as_str(): "fn main() -> bool {\n    let a = 4 < 5 && false;\n    a\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe07c4ea020,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
        ),
        start: 90,
        end: 94,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

