

TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 57,
            end: 58,
            as_str(): "N",
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
            21,
        ),
        span: Span {
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 61,
            end: 63,
            as_str(): "10",
        },
    },
    visibility: Private,
    return_type: TypeId(
        21,
    ),
    is_configurable: false,
    attributes: {},
    type_ascription_span: None,
    span: Span {
        src (ptr): 0x00007fb12b9f78e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
        ),
        start: 51,
        end: 64,
        as_str(): "const N = 10;",
    },
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 69,
            end: 86,
            as_str(): "simple_break_test",
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
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 103,
                                    end: 104,
                                    as_str(): "i",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 107,
                                    end: 108,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31635,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 95,
                    end: 109,
                    as_str(): "let mut i = 0;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: WhileLoop {
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
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 120,
                                    end: 124,
                                    as_str(): "true",
                                },
                            },
                            body: TyCodeBlock {
                                contents: [
                                    TyAstNode {
                                        content: Expression(
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
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 140,
                                                                            end: 142,
                                                                            as_str(): ">=",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 140,
                                                                            end: 142,
                                                                            as_str(): ">=",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "ge",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 140,
                                                                        end: 142,
                                                                        as_str(): ">=",
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
                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 11050,
                                                                            end: 11054,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 103,
                                                                                    end: 104,
                                                                                    as_str(): "i",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 138,
                                                                                end: 139,
                                                                                as_str(): "i",
                                                                            },
                                                                            mutability: Mutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 138,
                                                                            end: 139,
                                                                            as_str(): "i",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 11056,
                                                                            end: 11061,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 57,
                                                                                    end: 58,
                                                                                    as_str(): "N",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 143,
                                                                                end: 144,
                                                                                as_str(): "N",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 143,
                                                                            end: 144,
                                                                            as_str(): "N",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13315,
                                                                Span {
                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 11044,
                                                                    end: 11125,
                                                                    as_str(): "fn ge(self, other: Self) -> bool {\n        self.gt(other) || self.eq(other)\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 140,
                                                                        end: 142,
                                                                        as_str(): ">=",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 138,
                                                            end: 144,
                                                            as_str(): "i >= N",
                                                        },
                                                    },
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: Expression(
                                                                            TyExpression {
                                                                                expression: Break,
                                                                                return_type: TypeId(
                                                                                    31644,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 159,
                                                                                    end: 164,
                                                                                    as_str(): "break",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 159,
                                                                            end: 164,
                                                                            as_str(): "break",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31646,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 145,
                                                            end: 175,
                                                            as_str(): "{\n            break;\n        }",
                                                        },
                                                    },
                                                    else: None,
                                                },
                                                return_type: TypeId(
                                                    31649,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 135,
                                                    end: 175,
                                                    as_str(): "if i >= N {\n            break;\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 135,
                                            end: 175,
                                            as_str(): "if i >= N {\n            break;\n        }",
                                        },
                                    },
                                    TyAstNode {
                                        content: ImplicitReturnExpression(
                                            TyExpression {
                                                expression: Reassignment(
                                                    TyReassignment {
                                                        lhs_base_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 184,
                                                                end: 185,
                                                                as_str(): "i",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        lhs_type: TypeId(
                                                            21,
                                                        ),
                                                        lhs_indices: [],
                                                        rhs: TyExpression {
                                                            expression: FunctionApplication {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 186,
                                                                                end: 188,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 186,
                                                                                end: 188,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "add",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 186,
                                                                            end: 188,
                                                                            as_str(): "+=",
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
                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 124,
                                                                                end: 128,
                                                                                as_str(): "self",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        TyExpression {
                                                                            expression: VariableExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 103,
                                                                                        end: 104,
                                                                                        as_str(): "i",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 184,
                                                                                    end: 185,
                                                                                    as_str(): "i",
                                                                                },
                                                                                mutability: Mutable,
                                                                            },
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 184,
                                                                                end: 185,
                                                                                as_str(): "i",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 130,
                                                                                end: 135,
                                                                                as_str(): "other",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
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
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 189,
                                                                                end: 190,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                function_decl_id: DeclId(
                                                                    13316,
                                                                    Span {
                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 185,
                                                                        as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                    },
                                                                ),
                                                                self_state_idx: None,
                                                                selector: None,
                                                                type_binding: Some(
                                                                    TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 186,
                                                                            end: 188,
                                                                            as_str(): "+=",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 184,
                                                                end: 190,
                                                                as_str(): "i += 1",
                                                            },
                                                        },
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    31655,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 184,
                                                    end: 190,
                                                    as_str(): "i += 1",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 184,
                                            end: 190,
                                            as_str(): "i += 1",
                                        },
                                    },
                                ],
                            },
                        },
                        return_type: TypeId(
                            31656,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 114,
                            end: 196,
                            as_str(): "while true {\n        if i >= N {\n            break;\n        }\n        i += 1\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 114,
                    end: 196,
                    as_str(): "while true {\n        if i >= N {\n            break;\n        }\n        i += 1\n    }",
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
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 201,
                                        end: 207,
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
                                            src (ptr): 0x00007fb133074f40,
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
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 210,
                                                            end: 212,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 210,
                                                            end: 212,
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
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 210,
                                                        end: 212,
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
                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 103,
                                                                    end: 104,
                                                                    as_str(): "i",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 208,
                                                                end: 209,
                                                                as_str(): "i",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 208,
                                                            end: 209,
                                                            as_str(): "i",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 57,
                                                                    end: 58,
                                                                    as_str(): "N",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 213,
                                                                end: 214,
                                                                as_str(): "N",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 213,
                                                            end: 214,
                                                            as_str(): "N",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13318,
                                                Span {
                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 210,
                                                        end: 212,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 208,
                                            end: 214,
                                            as_str(): "i == N",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13319,
                                Span {
                                    src (ptr): 0x00007fb133074f40,
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
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 201,
                                        end: 207,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31661,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 201,
                            end: 215,
                            as_str(): "assert(i == N)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 201,
                    end: 215,
                    as_str(): "assert(i == N)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb12b9f78e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
        ),
        start: 66,
        end: 218,
        as_str(): "fn simple_break_test() {\n    let mut i = 0;\n    while true {\n        if i >= N {\n            break;\n        }\n        i += 1\n    }\n    assert(i == N);\n}",
    },
    attributes: {},
    return_type: TypeId(
        31634,
    ),
    initial_return_type: TypeId(
        31633,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb12b9f78e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
        ),
        start: 66,
        end: 88,
        as_str(): "fn simple_break_test()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 223,
            end: 243,
            as_str(): "simple_continue_test",
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
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 260,
                                    end: 261,
                                    as_str(): "i",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 264,
                                    end: 265,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31666,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 252,
                    end: 266,
                    as_str(): "let mut i = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 279,
                                    end: 282,
                                    as_str(): "sum",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 285,
                                    end: 286,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31668,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 271,
                    end: 287,
                    as_str(): "let mut sum = 0;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: WhileLoop {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 300,
                                                    end: 301,
                                                    as_str(): "<",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 300,
                                                    end: 301,
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
                                                src (ptr): 0x00007fb12b9f78e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                ),
                                                start: 300,
                                                end: 301,
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
                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 260,
                                                            end: 261,
                                                            as_str(): "i",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 298,
                                                        end: 299,
                                                        as_str(): "i",
                                                    },
                                                    mutability: Mutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 298,
                                                    end: 299,
                                                    as_str(): "i",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 57,
                                                            end: 58,
                                                            as_str(): "N",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 302,
                                                        end: 303,
                                                        as_str(): "N",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 302,
                                                    end: 303,
                                                    as_str(): "N",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13321,
                                        Span {
                                            src (ptr): 0x00007fb1357e0a80,
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
                                                src (ptr): 0x00007fb12b9f78e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                ),
                                                start: 300,
                                                end: 301,
                                                as_str(): "<",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 298,
                                    end: 303,
                                    as_str(): "i < N",
                                },
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
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 314,
                                                                end: 315,
                                                                as_str(): "i",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        lhs_type: TypeId(
                                                            21,
                                                        ),
                                                        lhs_indices: [],
                                                        rhs: TyExpression {
                                                            expression: FunctionApplication {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 316,
                                                                                end: 318,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 316,
                                                                                end: 318,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "add",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 316,
                                                                            end: 318,
                                                                            as_str(): "+=",
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
                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 124,
                                                                                end: 128,
                                                                                as_str(): "self",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        TyExpression {
                                                                            expression: VariableExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 260,
                                                                                        end: 261,
                                                                                        as_str(): "i",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 314,
                                                                                    end: 315,
                                                                                    as_str(): "i",
                                                                                },
                                                                                mutability: Mutable,
                                                                            },
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 314,
                                                                                end: 315,
                                                                                as_str(): "i",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 130,
                                                                                end: 135,
                                                                                as_str(): "other",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
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
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 319,
                                                                                end: 320,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                function_decl_id: DeclId(
                                                                    13322,
                                                                    Span {
                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 185,
                                                                        as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                    },
                                                                ),
                                                                self_state_idx: None,
                                                                selector: None,
                                                                type_binding: Some(
                                                                    TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 316,
                                                                            end: 318,
                                                                            as_str(): "+=",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 314,
                                                                end: 320,
                                                                as_str(): "i += 1",
                                                            },
                                                        },
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    31680,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 314,
                                                    end: 320,
                                                    as_str(): "i += 1",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 314,
                                            end: 320,
                                            as_str(): "i += 1",
                                        },
                                    },
                                    TyAstNode {
                                        content: Expression(
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
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 339,
                                                                            end: 341,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 339,
                                                                            end: 341,
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
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 339,
                                                                        end: 341,
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
                                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                        expression: FunctionApplication {
                                                                            call_path: CallPath {
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "core",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 335,
                                                                                            end: 336,
                                                                                            as_str(): "%",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 335,
                                                                                            end: 336,
                                                                                            as_str(): "%",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "modulo",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 335,
                                                                                        end: 336,
                                                                                        as_str(): "%",
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
                                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 2008,
                                                                                            end: 2012,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 260,
                                                                                                    end: 261,
                                                                                                    as_str(): "i",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 333,
                                                                                                end: 334,
                                                                                                as_str(): "i",
                                                                                            },
                                                                                            mutability: Mutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 333,
                                                                                            end: 334,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 2014,
                                                                                            end: 2019,
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
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 337,
                                                                                            end: 338,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            function_decl_id: DeclId(
                                                                                13323,
                                                                                Span {
                                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 1998,
                                                                                    end: 2137,
                                                                                    as_str(): "fn modulo(self, other: Self) -> Self {\n        asm(r1: self, r2: other, r3) {\n            mod r3 r1 r2;\n            r3: u64\n        }\n    }",
                                                                                },
                                                                            ),
                                                                            self_state_idx: None,
                                                                            selector: None,
                                                                            type_binding: Some(
                                                                                TypeBinding {
                                                                                    inner: (),
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 335,
                                                                                        end: 336,
                                                                                        as_str(): "%",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 333,
                                                                            end: 338,
                                                                            as_str(): "i % 2",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 342,
                                                                            end: 343,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13324,
                                                                Span {
                                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 339,
                                                                        end: 341,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 333,
                                                            end: 343,
                                                            as_str(): "i % 2 == 0",
                                                        },
                                                    },
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: Expression(
                                                                            TyExpression {
                                                                                expression: Continue,
                                                                                return_type: TypeId(
                                                                                    31690,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 358,
                                                                                    end: 366,
                                                                                    as_str(): "continue",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 358,
                                                                            end: 366,
                                                                            as_str(): "continue",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31692,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 344,
                                                            end: 377,
                                                            as_str(): "{\n            continue;\n        }",
                                                        },
                                                    },
                                                    else: None,
                                                },
                                                return_type: TypeId(
                                                    31695,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 330,
                                                    end: 377,
                                                    as_str(): "if i % 2 == 0 {\n            continue;\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 330,
                                            end: 377,
                                            as_str(): "if i % 2 == 0 {\n            continue;\n        }",
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
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 386,
                                                                end: 389,
                                                                as_str(): "sum",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        lhs_type: TypeId(
                                                            21,
                                                        ),
                                                        lhs_indices: [],
                                                        rhs: TyExpression {
                                                            expression: FunctionApplication {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 390,
                                                                                end: 392,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 390,
                                                                                end: 392,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "add",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 390,
                                                                            end: 392,
                                                                            as_str(): "+=",
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
                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 124,
                                                                                end: 128,
                                                                                as_str(): "self",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        TyExpression {
                                                                            expression: VariableExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 279,
                                                                                        end: 282,
                                                                                        as_str(): "sum",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 386,
                                                                                    end: 389,
                                                                                    as_str(): "sum",
                                                                                },
                                                                                mutability: Mutable,
                                                                            },
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 386,
                                                                                end: 389,
                                                                                as_str(): "sum",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 130,
                                                                                end: 135,
                                                                                as_str(): "other",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
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
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 393,
                                                                                end: 394,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                function_decl_id: DeclId(
                                                                    13325,
                                                                    Span {
                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 185,
                                                                        as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                    },
                                                                ),
                                                                self_state_idx: None,
                                                                selector: None,
                                                                type_binding: Some(
                                                                    TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 390,
                                                                            end: 392,
                                                                            as_str(): "+=",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 386,
                                                                end: 394,
                                                                as_str(): "sum += 1",
                                                            },
                                                        },
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    31702,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 386,
                                                    end: 394,
                                                    as_str(): "sum += 1",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 386,
                                            end: 394,
                                            as_str(): "sum += 1",
                                        },
                                    },
                                ],
                            },
                        },
                        return_type: TypeId(
                            31704,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 292,
                            end: 401,
                            as_str(): "while i < N {\n        i += 1;\n        if i % 2 == 0 {\n            continue;\n        }\n        sum += 1;\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 292,
                    end: 401,
                    as_str(): "while i < N {\n        i += 1;\n        if i % 2 == 0 {\n            continue;\n        }\n        sum += 1;\n    }",
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
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 406,
                                        end: 412,
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
                                            src (ptr): 0x00007fb133074f40,
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
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 417,
                                                            end: 419,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 417,
                                                            end: 419,
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
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 417,
                                                        end: 419,
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
                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 279,
                                                                    end: 282,
                                                                    as_str(): "sum",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 413,
                                                                end: 416,
                                                                as_str(): "sum",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 413,
                                                            end: 416,
                                                            as_str(): "sum",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 422,
                                                                            end: 423,
                                                                            as_str(): "/",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 422,
                                                                            end: 423,
                                                                            as_str(): "/",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "divide",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 422,
                                                                        end: 423,
                                                                        as_str(): "/",
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
                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 1547,
                                                                            end: 1551,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 57,
                                                                                    end: 58,
                                                                                    as_str(): "N",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 420,
                                                                                end: 421,
                                                                                as_str(): "N",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 420,
                                                                            end: 421,
                                                                            as_str(): "N",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 1553,
                                                                            end: 1558,
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
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 424,
                                                                            end: 425,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13327,
                                                                Span {
                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 1537,
                                                                    end: 1608,
                                                                    as_str(): "fn divide(self, other: Self) -> Self {\n        __div(self, other)\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 422,
                                                                        end: 423,
                                                                        as_str(): "/",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 420,
                                                            end: 425,
                                                            as_str(): "N / 2",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13328,
                                                Span {
                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 417,
                                                        end: 419,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 413,
                                            end: 425,
                                            as_str(): "sum == N / 2",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13329,
                                Span {
                                    src (ptr): 0x00007fb133074f40,
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
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 406,
                                        end: 412,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31712,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 406,
                            end: 426,
                            as_str(): "assert(sum == N / 2)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 406,
                    end: 426,
                    as_str(): "assert(sum == N / 2)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb12b9f78e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
        ),
        start: 220,
        end: 429,
        as_str(): "fn simple_continue_test() {\n    let mut i = 0;\n    let mut sum = 0;\n    while i < N {\n        i += 1;\n        if i % 2 == 0 {\n            continue;\n        }\n        sum += 1;\n    }\n    assert(sum == N / 2);\n}",
    },
    attributes: {},
    return_type: TypeId(
        31665,
    ),
    initial_return_type: TypeId(
        31664,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb12b9f78e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
        ),
        start: 220,
        end: 245,
        as_str(): "fn simple_continue_test()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 434,
            end: 457,
            as_str(): "break_and_continue_test",
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
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 474,
                                    end: 475,
                                    as_str(): "i",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 478,
                                    end: 479,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31717,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 466,
                    end: 480,
                    as_str(): "let mut i = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 493,
                                    end: 494,
                                    as_str(): "j",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 497,
                                    end: 498,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31719,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 485,
                    end: 499,
                    as_str(): "let mut j = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 512,
                                    end: 513,
                                    as_str(): "k",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 516,
                                    end: 517,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31721,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 504,
                    end: 518,
                    as_str(): "let mut k = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 531,
                                    end: 532,
                                    as_str(): "n",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 535,
                                    end: 536,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31723,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 523,
                    end: 537,
                    as_str(): "let mut n = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 550,
                                    end: 554,
                                    as_str(): "sum1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 557,
                                    end: 558,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31725,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 542,
                    end: 559,
                    as_str(): "let mut sum1 = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 572,
                                    end: 576,
                                    as_str(): "sum2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 579,
                                    end: 580,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31727,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 564,
                    end: 581,
                    as_str(): "let mut sum2 = 0;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: WhileLoop {
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
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 592,
                                    end: 596,
                                    as_str(): "true",
                                },
                            },
                            body: TyCodeBlock {
                                contents: [
                                    TyAstNode {
                                        content: Expression(
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
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 612,
                                                                            end: 614,
                                                                            as_str(): ">=",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 612,
                                                                            end: 614,
                                                                            as_str(): ">=",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "ge",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 612,
                                                                        end: 614,
                                                                        as_str(): ">=",
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
                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 11050,
                                                                            end: 11054,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 474,
                                                                                    end: 475,
                                                                                    as_str(): "i",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 610,
                                                                                end: 611,
                                                                                as_str(): "i",
                                                                            },
                                                                            mutability: Mutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 610,
                                                                            end: 611,
                                                                            as_str(): "i",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 11056,
                                                                            end: 11061,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 57,
                                                                                    end: 58,
                                                                                    as_str(): "N",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 615,
                                                                                end: 616,
                                                                                as_str(): "N",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 615,
                                                                            end: 616,
                                                                            as_str(): "N",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13331,
                                                                Span {
                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 11044,
                                                                    end: 11125,
                                                                    as_str(): "fn ge(self, other: Self) -> bool {\n        self.gt(other) || self.eq(other)\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 612,
                                                                        end: 614,
                                                                        as_str(): ">=",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 610,
                                                            end: 616,
                                                            as_str(): "i >= N",
                                                        },
                                                    },
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: Expression(
                                                                            TyExpression {
                                                                                expression: Break,
                                                                                return_type: TypeId(
                                                                                    31736,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 631,
                                                                                    end: 636,
                                                                                    as_str(): "break",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 631,
                                                                            end: 636,
                                                                            as_str(): "break",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31738,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 617,
                                                            end: 647,
                                                            as_str(): "{\n            break;\n        }",
                                                        },
                                                    },
                                                    else: None,
                                                },
                                                return_type: TypeId(
                                                    31741,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 607,
                                                    end: 647,
                                                    as_str(): "if i >= N {\n            break;\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 607,
                                            end: 647,
                                            as_str(): "if i >= N {\n            break;\n        }",
                                        },
                                    },
                                    TyAstNode {
                                        content: Expression(
                                            TyExpression {
                                                expression: WhileLoop {
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
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 662,
                                                            end: 666,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                    body: TyCodeBlock {
                                                        contents: [
                                                            TyAstNode {
                                                                content: Expression(
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
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 686,
                                                                                                    end: 688,
                                                                                                    as_str(): ">=",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 686,
                                                                                                    end: 688,
                                                                                                    as_str(): ">=",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ge",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 686,
                                                                                                end: 688,
                                                                                                as_str(): ">=",
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
                                                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                    ),
                                                                                                    start: 11050,
                                                                                                    end: 11054,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 493,
                                                                                                            end: 494,
                                                                                                            as_str(): "j",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 684,
                                                                                                        end: 685,
                                                                                                        as_str(): "j",
                                                                                                    },
                                                                                                    mutability: Mutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 684,
                                                                                                    end: 685,
                                                                                                    as_str(): "j",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        (
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                    ),
                                                                                                    start: 11056,
                                                                                                    end: 11061,
                                                                                                    as_str(): "other",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 57,
                                                                                                            end: 58,
                                                                                                            as_str(): "N",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 689,
                                                                                                        end: 690,
                                                                                                        as_str(): "N",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 689,
                                                                                                    end: 690,
                                                                                                    as_str(): "N",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    function_decl_id: DeclId(
                                                                                        13332,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 11044,
                                                                                            end: 11125,
                                                                                            as_str(): "fn ge(self, other: Self) -> bool {\n        self.gt(other) || self.eq(other)\n    }",
                                                                                        },
                                                                                    ),
                                                                                    self_state_idx: None,
                                                                                    selector: None,
                                                                                    type_binding: Some(
                                                                                        TypeBinding {
                                                                                            inner: (),
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 686,
                                                                                                end: 688,
                                                                                                as_str(): ">=",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 684,
                                                                                    end: 690,
                                                                                    as_str(): "j >= N",
                                                                                },
                                                                            },
                                                                            then: TyExpression {
                                                                                expression: CodeBlock(
                                                                                    TyCodeBlock {
                                                                                        contents: [
                                                                                            TyAstNode {
                                                                                                content: Expression(
                                                                                                    TyExpression {
                                                                                                        expression: Break,
                                                                                                        return_type: TypeId(
                                                                                                            31749,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 709,
                                                                                                            end: 714,
                                                                                                            as_str(): "break",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 709,
                                                                                                    end: 714,
                                                                                                    as_str(): "break",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    31751,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 691,
                                                                                    end: 729,
                                                                                    as_str(): "{\n                break;\n            }",
                                                                                },
                                                                            },
                                                                            else: None,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31754,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 681,
                                                                            end: 729,
                                                                            as_str(): "if j >= N {\n                break;\n            }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 681,
                                                                    end: 729,
                                                                    as_str(): "if j >= N {\n                break;\n            }",
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
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 742,
                                                                                        end: 746,
                                                                                        as_str(): "sum1",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                lhs_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                lhs_indices: [],
                                                                                rhs: TyExpression {
                                                                                    expression: FunctionApplication {
                                                                                        call_path: CallPath {
                                                                                            prefixes: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "core",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 747,
                                                                                                        end: 749,
                                                                                                        as_str(): "+=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 747,
                                                                                                        end: 749,
                                                                                                        as_str(): "+=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "add",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 747,
                                                                                                    end: 749,
                                                                                                    as_str(): "+=",
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
                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 124,
                                                                                                        end: 128,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: VariableExpression {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 550,
                                                                                                                end: 554,
                                                                                                                as_str(): "sum1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 742,
                                                                                                            end: 746,
                                                                                                            as_str(): "sum1",
                                                                                                        },
                                                                                                        mutability: Mutable,
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 742,
                                                                                                        end: 746,
                                                                                                        as_str(): "sum1",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 130,
                                                                                                        end: 135,
                                                                                                        as_str(): "other",
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
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 756,
                                                                                                                        end: 757,
                                                                                                                        as_str(): "+",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 756,
                                                                                                                        end: 757,
                                                                                                                        as_str(): "+",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "add",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 756,
                                                                                                                    end: 757,
                                                                                                                    as_str(): "+",
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
                                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                                        path: Some(
                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                        ),
                                                                                                                        start: 124,
                                                                                                                        end: 128,
                                                                                                                        as_str(): "self",
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
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 752,
                                                                                                                                        end: 753,
                                                                                                                                        as_str(): "*",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: Some(
                                                                                                                                        "ops",
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 752,
                                                                                                                                        end: 753,
                                                                                                                                        as_str(): "*",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ],
                                                                                                                            suffix: BaseIdent {
                                                                                                                                name_override_opt: Some(
                                                                                                                                    "multiply",
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 752,
                                                                                                                                    end: 753,
                                                                                                                                    as_str(): "*",
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
                                                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                                                        path: Some(
                                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1068,
                                                                                                                                        end: 1072,
                                                                                                                                        as_str(): "self",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                TyExpression {
                                                                                                                                    expression: VariableExpression {
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 474,
                                                                                                                                                end: 475,
                                                                                                                                                as_str(): "i",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 750,
                                                                                                                                            end: 751,
                                                                                                                                            as_str(): "i",
                                                                                                                                        },
                                                                                                                                        mutability: Mutable,
                                                                                                                                    },
                                                                                                                                    return_type: TypeId(
                                                                                                                                        21,
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 750,
                                                                                                                                        end: 751,
                                                                                                                                        as_str(): "i",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            (
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                                                        path: Some(
                                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1074,
                                                                                                                                        end: 1079,
                                                                                                                                        as_str(): "other",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                TyExpression {
                                                                                                                                    expression: VariableExpression {
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 57,
                                                                                                                                                end: 58,
                                                                                                                                                as_str(): "N",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 754,
                                                                                                                                            end: 755,
                                                                                                                                            as_str(): "N",
                                                                                                                                        },
                                                                                                                                        mutability: Immutable,
                                                                                                                                    },
                                                                                                                                    return_type: TypeId(
                                                                                                                                        21,
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 754,
                                                                                                                                        end: 755,
                                                                                                                                        as_str(): "N",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ],
                                                                                                                        function_decl_id: DeclId(
                                                                                                                            13333,
                                                                                                                            Span {
                                                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                                                path: Some(
                                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                                ),
                                                                                                                                start: 1056,
                                                                                                                                end: 1129,
                                                                                                                                as_str(): "fn multiply(self, other: Self) -> Self {\n        __mul(self, other)\n    }",
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        self_state_idx: None,
                                                                                                                        selector: None,
                                                                                                                        type_binding: Some(
                                                                                                                            TypeBinding {
                                                                                                                                inner: (),
                                                                                                                                type_arguments: [],
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 752,
                                                                                                                                    end: 753,
                                                                                                                                    as_str(): "*",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    },
                                                                                                                    return_type: TypeId(
                                                                                                                        21,
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 750,
                                                                                                                        end: 755,
                                                                                                                        as_str(): "i * N",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            (
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                                        path: Some(
                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                        ),
                                                                                                                        start: 130,
                                                                                                                        end: 135,
                                                                                                                        as_str(): "other",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                TyExpression {
                                                                                                                    expression: VariableExpression {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 493,
                                                                                                                                end: 494,
                                                                                                                                as_str(): "j",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 758,
                                                                                                                            end: 759,
                                                                                                                            as_str(): "j",
                                                                                                                        },
                                                                                                                        mutability: Mutable,
                                                                                                                    },
                                                                                                                    return_type: TypeId(
                                                                                                                        21,
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 758,
                                                                                                                        end: 759,
                                                                                                                        as_str(): "j",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        ],
                                                                                                        function_decl_id: DeclId(
                                                                                                            13334,
                                                                                                            Span {
                                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                                path: Some(
                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                ),
                                                                                                                start: 117,
                                                                                                                end: 185,
                                                                                                                as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                                                            },
                                                                                                        ),
                                                                                                        self_state_idx: None,
                                                                                                        selector: None,
                                                                                                        type_binding: Some(
                                                                                                            TypeBinding {
                                                                                                                inner: (),
                                                                                                                type_arguments: [],
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 756,
                                                                                                                    end: 757,
                                                                                                                    as_str(): "+",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 750,
                                                                                                        end: 759,
                                                                                                        as_str(): "i * N + j",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            13335,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                ),
                                                                                                start: 117,
                                                                                                end: 185,
                                                                                                as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                                            },
                                                                                        ),
                                                                                        self_state_idx: None,
                                                                                        selector: None,
                                                                                        type_binding: Some(
                                                                                            TypeBinding {
                                                                                                inner: (),
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 747,
                                                                                                    end: 749,
                                                                                                    as_str(): "+=",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 742,
                                                                                        end: 759,
                                                                                        as_str(): "sum1 += i * N + j",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31764,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 742,
                                                                            end: 759,
                                                                            as_str(): "sum1 += i * N + j",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 742,
                                                                    end: 759,
                                                                    as_str(): "sum1 += i * N + j",
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
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 773,
                                                                                        end: 774,
                                                                                        as_str(): "j",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                lhs_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                lhs_indices: [],
                                                                                rhs: TyExpression {
                                                                                    expression: FunctionApplication {
                                                                                        call_path: CallPath {
                                                                                            prefixes: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "core",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 775,
                                                                                                        end: 777,
                                                                                                        as_str(): "+=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 775,
                                                                                                        end: 777,
                                                                                                        as_str(): "+=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "add",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 775,
                                                                                                    end: 777,
                                                                                                    as_str(): "+=",
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
                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 124,
                                                                                                        end: 128,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: VariableExpression {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 493,
                                                                                                                end: 494,
                                                                                                                as_str(): "j",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 773,
                                                                                                            end: 774,
                                                                                                            as_str(): "j",
                                                                                                        },
                                                                                                        mutability: Mutable,
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 773,
                                                                                                        end: 774,
                                                                                                        as_str(): "j",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 130,
                                                                                                        end: 135,
                                                                                                        as_str(): "other",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
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
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 778,
                                                                                                        end: 779,
                                                                                                        as_str(): "1",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            13336,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                ),
                                                                                                start: 117,
                                                                                                end: 185,
                                                                                                as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                                            },
                                                                                        ),
                                                                                        self_state_idx: None,
                                                                                        selector: None,
                                                                                        type_binding: Some(
                                                                                            TypeBinding {
                                                                                                inner: (),
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 775,
                                                                                                    end: 777,
                                                                                                    as_str(): "+=",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 773,
                                                                                        end: 779,
                                                                                        as_str(): "j += 1",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31771,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 773,
                                                                            end: 779,
                                                                            as_str(): "j += 1",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 773,
                                                                    end: 779,
                                                                    as_str(): "j += 1",
                                                                },
                                                            },
                                                            TyAstNode {
                                                                content: Expression(
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
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 803,
                                                                                                    end: 805,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 803,
                                                                                                    end: 805,
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
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 803,
                                                                                                end: 805,
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
                                                                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                                                                expression: FunctionApplication {
                                                                                                    call_path: CallPath {
                                                                                                        prefixes: [
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "core",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 799,
                                                                                                                    end: 800,
                                                                                                                    as_str(): "%",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "ops",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 799,
                                                                                                                    end: 800,
                                                                                                                    as_str(): "%",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ],
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "modulo",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 799,
                                                                                                                end: 800,
                                                                                                                as_str(): "%",
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
                                                                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                    ),
                                                                                                                    start: 2008,
                                                                                                                    end: 2012,
                                                                                                                    as_str(): "self",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            TyExpression {
                                                                                                                expression: VariableExpression {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 493,
                                                                                                                            end: 494,
                                                                                                                            as_str(): "j",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 797,
                                                                                                                        end: 798,
                                                                                                                        as_str(): "j",
                                                                                                                    },
                                                                                                                    mutability: Mutable,
                                                                                                                },
                                                                                                                return_type: TypeId(
                                                                                                                    21,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 797,
                                                                                                                    end: 798,
                                                                                                                    as_str(): "j",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        (
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                    ),
                                                                                                                    start: 2014,
                                                                                                                    end: 2019,
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
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 801,
                                                                                                                    end: 802,
                                                                                                                    as_str(): "2",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ],
                                                                                                    function_decl_id: DeclId(
                                                                                                        13337,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                                                            path: Some(
                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                            ),
                                                                                                            start: 1998,
                                                                                                            end: 2137,
                                                                                                            as_str(): "fn modulo(self, other: Self) -> Self {\n        asm(r1: self, r2: other, r3) {\n            mod r3 r1 r2;\n            r3: u64\n        }\n    }",
                                                                                                        },
                                                                                                    ),
                                                                                                    self_state_idx: None,
                                                                                                    selector: None,
                                                                                                    type_binding: Some(
                                                                                                        TypeBinding {
                                                                                                            inner: (),
                                                                                                            type_arguments: [],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 799,
                                                                                                                end: 800,
                                                                                                                as_str(): "%",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 797,
                                                                                                    end: 802,
                                                                                                    as_str(): "j % 2",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        (
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                                                                        0,
                                                                                                    ),
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 806,
                                                                                                    end: 807,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    function_decl_id: DeclId(
                                                                                        13338,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 803,
                                                                                                end: 805,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 797,
                                                                                    end: 807,
                                                                                    as_str(): "j % 2 == 0",
                                                                                },
                                                                            },
                                                                            then: TyExpression {
                                                                                expression: CodeBlock(
                                                                                    TyCodeBlock {
                                                                                        contents: [
                                                                                            TyAstNode {
                                                                                                content: Expression(
                                                                                                    TyExpression {
                                                                                                        expression: Continue,
                                                                                                        return_type: TypeId(
                                                                                                            31781,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 826,
                                                                                                            end: 834,
                                                                                                            as_str(): "continue",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 826,
                                                                                                    end: 834,
                                                                                                    as_str(): "continue",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    31783,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 808,
                                                                                    end: 849,
                                                                                    as_str(): "{\n                continue;\n            }",
                                                                                },
                                                                            },
                                                                            else: None,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31786,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 794,
                                                                            end: 849,
                                                                            as_str(): "if j % 2 == 0 {\n                continue;\n            }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 794,
                                                                    end: 849,
                                                                    as_str(): "if j % 2 == 0 {\n                continue;\n            }",
                                                                },
                                                            },
                                                            TyAstNode {
                                                                content: ImplicitReturnExpression(
                                                                    TyExpression {
                                                                        expression: WhileLoop {
                                                                            condition: TyExpression {
                                                                                expression: FunctionApplication {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "core",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 871,
                                                                                                    end: 872,
                                                                                                    as_str(): "<",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 871,
                                                                                                    end: 872,
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
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 871,
                                                                                                end: 872,
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
                                                                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 531,
                                                                                                            end: 532,
                                                                                                            as_str(): "n",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 869,
                                                                                                        end: 870,
                                                                                                        as_str(): "n",
                                                                                                    },
                                                                                                    mutability: Mutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 869,
                                                                                                    end: 870,
                                                                                                    as_str(): "n",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        (
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 57,
                                                                                                            end: 58,
                                                                                                            as_str(): "N",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 873,
                                                                                                        end: 874,
                                                                                                        as_str(): "N",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 873,
                                                                                                    end: 874,
                                                                                                    as_str(): "N",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    function_decl_id: DeclId(
                                                                                        13339,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 871,
                                                                                                end: 872,
                                                                                                as_str(): "<",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 869,
                                                                                    end: 874,
                                                                                    as_str(): "n < N",
                                                                                },
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
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 893,
                                                                                                                end: 897,
                                                                                                                as_str(): "sum1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        lhs_type: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        lhs_indices: [],
                                                                                                        rhs: TyExpression {
                                                                                                            expression: FunctionApplication {
                                                                                                                call_path: CallPath {
                                                                                                                    prefixes: [
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: Some(
                                                                                                                                "core",
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 898,
                                                                                                                                end: 900,
                                                                                                                                as_str(): "+=",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: Some(
                                                                                                                                "ops",
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 898,
                                                                                                                                end: 900,
                                                                                                                                as_str(): "+=",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ],
                                                                                                                    suffix: BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "add",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 898,
                                                                                                                            end: 900,
                                                                                                                            as_str(): "+=",
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
                                                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                                                path: Some(
                                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                                ),
                                                                                                                                start: 124,
                                                                                                                                end: 128,
                                                                                                                                as_str(): "self",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        TyExpression {
                                                                                                                            expression: VariableExpression {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 550,
                                                                                                                                        end: 554,
                                                                                                                                        as_str(): "sum1",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 893,
                                                                                                                                    end: 897,
                                                                                                                                    as_str(): "sum1",
                                                                                                                                },
                                                                                                                                mutability: Mutable,
                                                                                                                            },
                                                                                                                            return_type: TypeId(
                                                                                                                                21,
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 893,
                                                                                                                                end: 897,
                                                                                                                                as_str(): "sum1",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    (
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                                                path: Some(
                                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                                ),
                                                                                                                                start: 130,
                                                                                                                                end: 135,
                                                                                                                                as_str(): "other",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        TyExpression {
                                                                                                                            expression: VariableExpression {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 531,
                                                                                                                                        end: 532,
                                                                                                                                        as_str(): "n",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 901,
                                                                                                                                    end: 902,
                                                                                                                                    as_str(): "n",
                                                                                                                                },
                                                                                                                                mutability: Mutable,
                                                                                                                            },
                                                                                                                            return_type: TypeId(
                                                                                                                                21,
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 901,
                                                                                                                                end: 902,
                                                                                                                                as_str(): "n",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ],
                                                                                                                function_decl_id: DeclId(
                                                                                                                    13340,
                                                                                                                    Span {
                                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                                        path: Some(
                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                        ),
                                                                                                                        start: 117,
                                                                                                                        end: 185,
                                                                                                                        as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                                                                    },
                                                                                                                ),
                                                                                                                self_state_idx: None,
                                                                                                                selector: None,
                                                                                                                type_binding: Some(
                                                                                                                    TypeBinding {
                                                                                                                        inner: (),
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 898,
                                                                                                                            end: 900,
                                                                                                                            as_str(): "+=",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                21,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 893,
                                                                                                                end: 902,
                                                                                                                as_str(): "sum1 += n",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    31795,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 893,
                                                                                                    end: 902,
                                                                                                    as_str(): "sum1 += n",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 893,
                                                                                            end: 902,
                                                                                            as_str(): "sum1 += n",
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
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 920,
                                                                                                                end: 921,
                                                                                                                as_str(): "n",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        lhs_type: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        lhs_indices: [],
                                                                                                        rhs: TyExpression {
                                                                                                            expression: FunctionApplication {
                                                                                                                call_path: CallPath {
                                                                                                                    prefixes: [
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: Some(
                                                                                                                                "core",
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 922,
                                                                                                                                end: 924,
                                                                                                                                as_str(): "+=",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: Some(
                                                                                                                                "ops",
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 922,
                                                                                                                                end: 924,
                                                                                                                                as_str(): "+=",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ],
                                                                                                                    suffix: BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "add",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 922,
                                                                                                                            end: 924,
                                                                                                                            as_str(): "+=",
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
                                                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                                                path: Some(
                                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                                ),
                                                                                                                                start: 124,
                                                                                                                                end: 128,
                                                                                                                                as_str(): "self",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        TyExpression {
                                                                                                                            expression: VariableExpression {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 531,
                                                                                                                                        end: 532,
                                                                                                                                        as_str(): "n",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 920,
                                                                                                                                    end: 921,
                                                                                                                                    as_str(): "n",
                                                                                                                                },
                                                                                                                                mutability: Mutable,
                                                                                                                            },
                                                                                                                            return_type: TypeId(
                                                                                                                                21,
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 920,
                                                                                                                                end: 921,
                                                                                                                                as_str(): "n",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    (
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                                                path: Some(
                                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                                ),
                                                                                                                                start: 130,
                                                                                                                                end: 135,
                                                                                                                                as_str(): "other",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
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
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 925,
                                                                                                                                end: 926,
                                                                                                                                as_str(): "1",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ],
                                                                                                                function_decl_id: DeclId(
                                                                                                                    13341,
                                                                                                                    Span {
                                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                                        path: Some(
                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                        ),
                                                                                                                        start: 117,
                                                                                                                        end: 185,
                                                                                                                        as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                                                                    },
                                                                                                                ),
                                                                                                                self_state_idx: None,
                                                                                                                selector: None,
                                                                                                                type_binding: Some(
                                                                                                                    TypeBinding {
                                                                                                                        inner: (),
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 922,
                                                                                                                            end: 924,
                                                                                                                            as_str(): "+=",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                21,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 920,
                                                                                                                end: 926,
                                                                                                                as_str(): "n += 1",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    31802,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 920,
                                                                                                    end: 926,
                                                                                                    as_str(): "n += 1",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 920,
                                                                                            end: 926,
                                                                                            as_str(): "n += 1",
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
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 952,
                                                                                                                            end: 953,
                                                                                                                            as_str(): ">",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "ops",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 952,
                                                                                                                            end: 953,
                                                                                                                            as_str(): ">",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ],
                                                                                                                suffix: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "gt",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 952,
                                                                                                                        end: 953,
                                                                                                                        as_str(): ">",
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
                                                                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                                                                            path: Some(
                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                            ),
                                                                                                                            start: 3860,
                                                                                                                            end: 3864,
                                                                                                                            as_str(): "self",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    TyExpression {
                                                                                                                        expression: VariableExpression {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 550,
                                                                                                                                    end: 554,
                                                                                                                                    as_str(): "sum1",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 947,
                                                                                                                                end: 951,
                                                                                                                                as_str(): "sum1",
                                                                                                                            },
                                                                                                                            mutability: Mutable,
                                                                                                                        },
                                                                                                                        return_type: TypeId(
                                                                                                                            21,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 947,
                                                                                                                            end: 951,
                                                                                                                            as_str(): "sum1",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                (
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                                                                            path: Some(
                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                            ),
                                                                                                                            start: 3866,
                                                                                                                            end: 3871,
                                                                                                                            as_str(): "other",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    TyExpression {
                                                                                                                        expression: Literal(
                                                                                                                            U64(
                                                                                                                                50,
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        return_type: TypeId(
                                                                                                                            21,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 954,
                                                                                                                            end: 956,
                                                                                                                            as_str(): "50",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ],
                                                                                                            function_decl_id: DeclId(
                                                                                                                13342,
                                                                                                                Span {
                                                                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                    ),
                                                                                                                    start: 3854,
                                                                                                                    end: 3989,
                                                                                                                    as_str(): "fn gt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            gt r3 r1 r2;\n            r3: bool\n        }\n    }",
                                                                                                                },
                                                                                                            ),
                                                                                                            self_state_idx: None,
                                                                                                            selector: None,
                                                                                                            type_binding: Some(
                                                                                                                TypeBinding {
                                                                                                                    inner: (),
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 952,
                                                                                                                        end: 953,
                                                                                                                        as_str(): ">",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            71,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 947,
                                                                                                            end: 956,
                                                                                                            as_str(): "sum1 > 50",
                                                                                                        },
                                                                                                    },
                                                                                                    then: TyExpression {
                                                                                                        expression: CodeBlock(
                                                                                                            TyCodeBlock {
                                                                                                                contents: [
                                                                                                                    TyAstNode {
                                                                                                                        content: Expression(
                                                                                                                            TyExpression {
                                                                                                                                expression: Break,
                                                                                                                                return_type: TypeId(
                                                                                                                                    31808,
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 979,
                                                                                                                                    end: 984,
                                                                                                                                    as_str(): "break",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 979,
                                                                                                                            end: 984,
                                                                                                                            as_str(): "break",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ],
                                                                                                            },
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            31810,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 957,
                                                                                                            end: 1003,
                                                                                                            as_str(): "{\n                    break;\n                }",
                                                                                                        },
                                                                                                    },
                                                                                                    else: None,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    31813,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 944,
                                                                                                    end: 1003,
                                                                                                    as_str(): "if sum1 > 50 {\n                    break;\n                }",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 944,
                                                                                            end: 1003,
                                                                                            as_str(): "if sum1 > 50 {\n                    break;\n                }",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31814,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 863,
                                                                            end: 1017,
                                                                            as_str(): "while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 863,
                                                                    end: 1017,
                                                                    as_str(): "while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                },
                                                return_type: TypeId(
                                                    31815,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 656,
                                                    end: 1027,
                                                    as_str(): "while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 656,
                                            end: 1027,
                                            as_str(): "while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }",
                                        },
                                    },
                                    TyAstNode {
                                        content: Expression(
                                            TyExpression {
                                                expression: WhileLoop {
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
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1043,
                                                            end: 1047,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                    body: TyCodeBlock {
                                                        contents: [
                                                            TyAstNode {
                                                                content: Expression(
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
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1067,
                                                                                                    end: 1069,
                                                                                                    as_str(): ">=",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1067,
                                                                                                    end: 1069,
                                                                                                    as_str(): ">=",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ge",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1067,
                                                                                                end: 1069,
                                                                                                as_str(): ">=",
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
                                                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                    ),
                                                                                                    start: 11050,
                                                                                                    end: 11054,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 512,
                                                                                                            end: 513,
                                                                                                            as_str(): "k",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1065,
                                                                                                        end: 1066,
                                                                                                        as_str(): "k",
                                                                                                    },
                                                                                                    mutability: Mutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1065,
                                                                                                    end: 1066,
                                                                                                    as_str(): "k",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        (
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                    ),
                                                                                                    start: 11056,
                                                                                                    end: 11061,
                                                                                                    as_str(): "other",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 57,
                                                                                                            end: 58,
                                                                                                            as_str(): "N",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1070,
                                                                                                        end: 1071,
                                                                                                        as_str(): "N",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1070,
                                                                                                    end: 1071,
                                                                                                    as_str(): "N",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    function_decl_id: DeclId(
                                                                                        13343,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 11044,
                                                                                            end: 11125,
                                                                                            as_str(): "fn ge(self, other: Self) -> bool {\n        self.gt(other) || self.eq(other)\n    }",
                                                                                        },
                                                                                    ),
                                                                                    self_state_idx: None,
                                                                                    selector: None,
                                                                                    type_binding: Some(
                                                                                        TypeBinding {
                                                                                            inner: (),
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1067,
                                                                                                end: 1069,
                                                                                                as_str(): ">=",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1065,
                                                                                    end: 1071,
                                                                                    as_str(): "k >= N",
                                                                                },
                                                                            },
                                                                            then: TyExpression {
                                                                                expression: CodeBlock(
                                                                                    TyCodeBlock {
                                                                                        contents: [
                                                                                            TyAstNode {
                                                                                                content: Expression(
                                                                                                    TyExpression {
                                                                                                        expression: Break,
                                                                                                        return_type: TypeId(
                                                                                                            31823,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1090,
                                                                                                            end: 1095,
                                                                                                            as_str(): "break",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1090,
                                                                                                    end: 1095,
                                                                                                    as_str(): "break",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    31825,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1072,
                                                                                    end: 1110,
                                                                                    as_str(): "{\n                break;\n            }",
                                                                                },
                                                                            },
                                                                            else: None,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31828,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1062,
                                                                            end: 1110,
                                                                            as_str(): "if k >= N {\n                break;\n            }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 1062,
                                                                    end: 1110,
                                                                    as_str(): "if k >= N {\n                break;\n            }",
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
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1123,
                                                                                        end: 1127,
                                                                                        as_str(): "sum1",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                lhs_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                lhs_indices: [],
                                                                                rhs: TyExpression {
                                                                                    expression: FunctionApplication {
                                                                                        call_path: CallPath {
                                                                                            prefixes: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "core",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1128,
                                                                                                        end: 1130,
                                                                                                        as_str(): "+=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1128,
                                                                                                        end: 1130,
                                                                                                        as_str(): "+=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "add",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1128,
                                                                                                    end: 1130,
                                                                                                    as_str(): "+=",
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
                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 124,
                                                                                                        end: 128,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: VariableExpression {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 550,
                                                                                                                end: 554,
                                                                                                                as_str(): "sum1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1123,
                                                                                                            end: 1127,
                                                                                                            as_str(): "sum1",
                                                                                                        },
                                                                                                        mutability: Mutable,
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1123,
                                                                                                        end: 1127,
                                                                                                        as_str(): "sum1",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 130,
                                                                                                        end: 135,
                                                                                                        as_str(): "other",
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
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1137,
                                                                                                                        end: 1138,
                                                                                                                        as_str(): "+",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1137,
                                                                                                                        end: 1138,
                                                                                                                        as_str(): "+",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "add",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1137,
                                                                                                                    end: 1138,
                                                                                                                    as_str(): "+",
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
                                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                                        path: Some(
                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                        ),
                                                                                                                        start: 124,
                                                                                                                        end: 128,
                                                                                                                        as_str(): "self",
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
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1133,
                                                                                                                                        end: 1134,
                                                                                                                                        as_str(): "*",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: Some(
                                                                                                                                        "ops",
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1133,
                                                                                                                                        end: 1134,
                                                                                                                                        as_str(): "*",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ],
                                                                                                                            suffix: BaseIdent {
                                                                                                                                name_override_opt: Some(
                                                                                                                                    "multiply",
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1133,
                                                                                                                                    end: 1134,
                                                                                                                                    as_str(): "*",
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
                                                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                                                        path: Some(
                                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1068,
                                                                                                                                        end: 1072,
                                                                                                                                        as_str(): "self",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                TyExpression {
                                                                                                                                    expression: VariableExpression {
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 474,
                                                                                                                                                end: 475,
                                                                                                                                                as_str(): "i",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1131,
                                                                                                                                            end: 1132,
                                                                                                                                            as_str(): "i",
                                                                                                                                        },
                                                                                                                                        mutability: Mutable,
                                                                                                                                    },
                                                                                                                                    return_type: TypeId(
                                                                                                                                        21,
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1131,
                                                                                                                                        end: 1132,
                                                                                                                                        as_str(): "i",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            (
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                                                        path: Some(
                                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1074,
                                                                                                                                        end: 1079,
                                                                                                                                        as_str(): "other",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                TyExpression {
                                                                                                                                    expression: VariableExpression {
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 57,
                                                                                                                                                end: 58,
                                                                                                                                                as_str(): "N",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1135,
                                                                                                                                            end: 1136,
                                                                                                                                            as_str(): "N",
                                                                                                                                        },
                                                                                                                                        mutability: Immutable,
                                                                                                                                    },
                                                                                                                                    return_type: TypeId(
                                                                                                                                        21,
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1135,
                                                                                                                                        end: 1136,
                                                                                                                                        as_str(): "N",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ],
                                                                                                                        function_decl_id: DeclId(
                                                                                                                            13344,
                                                                                                                            Span {
                                                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                                                path: Some(
                                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                                ),
                                                                                                                                start: 1056,
                                                                                                                                end: 1129,
                                                                                                                                as_str(): "fn multiply(self, other: Self) -> Self {\n        __mul(self, other)\n    }",
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        self_state_idx: None,
                                                                                                                        selector: None,
                                                                                                                        type_binding: Some(
                                                                                                                            TypeBinding {
                                                                                                                                inner: (),
                                                                                                                                type_arguments: [],
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1133,
                                                                                                                                    end: 1134,
                                                                                                                                    as_str(): "*",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    },
                                                                                                                    return_type: TypeId(
                                                                                                                        21,
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1131,
                                                                                                                        end: 1136,
                                                                                                                        as_str(): "i * N",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            (
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                                        path: Some(
                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                        ),
                                                                                                                        start: 130,
                                                                                                                        end: 135,
                                                                                                                        as_str(): "other",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                TyExpression {
                                                                                                                    expression: VariableExpression {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 512,
                                                                                                                                end: 513,
                                                                                                                                as_str(): "k",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1139,
                                                                                                                            end: 1140,
                                                                                                                            as_str(): "k",
                                                                                                                        },
                                                                                                                        mutability: Mutable,
                                                                                                                    },
                                                                                                                    return_type: TypeId(
                                                                                                                        21,
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1139,
                                                                                                                        end: 1140,
                                                                                                                        as_str(): "k",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        ],
                                                                                                        function_decl_id: DeclId(
                                                                                                            13345,
                                                                                                            Span {
                                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                                path: Some(
                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                ),
                                                                                                                start: 117,
                                                                                                                end: 185,
                                                                                                                as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                                                            },
                                                                                                        ),
                                                                                                        self_state_idx: None,
                                                                                                        selector: None,
                                                                                                        type_binding: Some(
                                                                                                            TypeBinding {
                                                                                                                inner: (),
                                                                                                                type_arguments: [],
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1137,
                                                                                                                    end: 1138,
                                                                                                                    as_str(): "+",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1131,
                                                                                                        end: 1140,
                                                                                                        as_str(): "i * N + k",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            13346,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                ),
                                                                                                start: 117,
                                                                                                end: 185,
                                                                                                as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                                            },
                                                                                        ),
                                                                                        self_state_idx: None,
                                                                                        selector: None,
                                                                                        type_binding: Some(
                                                                                            TypeBinding {
                                                                                                inner: (),
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1128,
                                                                                                    end: 1130,
                                                                                                    as_str(): "+=",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1123,
                                                                                        end: 1140,
                                                                                        as_str(): "sum1 += i * N + k",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31838,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1123,
                                                                            end: 1140,
                                                                            as_str(): "sum1 += i * N + k",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 1123,
                                                                    end: 1140,
                                                                    as_str(): "sum1 += i * N + k",
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
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1154,
                                                                                        end: 1155,
                                                                                        as_str(): "k",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                lhs_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                lhs_indices: [],
                                                                                rhs: TyExpression {
                                                                                    expression: FunctionApplication {
                                                                                        call_path: CallPath {
                                                                                            prefixes: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "core",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1156,
                                                                                                        end: 1158,
                                                                                                        as_str(): "+=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1156,
                                                                                                        end: 1158,
                                                                                                        as_str(): "+=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "add",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1156,
                                                                                                    end: 1158,
                                                                                                    as_str(): "+=",
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
                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 124,
                                                                                                        end: 128,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: VariableExpression {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 512,
                                                                                                                end: 513,
                                                                                                                as_str(): "k",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1154,
                                                                                                            end: 1155,
                                                                                                            as_str(): "k",
                                                                                                        },
                                                                                                        mutability: Mutable,
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1154,
                                                                                                        end: 1155,
                                                                                                        as_str(): "k",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 130,
                                                                                                        end: 135,
                                                                                                        as_str(): "other",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
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
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1159,
                                                                                                        end: 1160,
                                                                                                        as_str(): "1",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            13347,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                ),
                                                                                                start: 117,
                                                                                                end: 185,
                                                                                                as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                                            },
                                                                                        ),
                                                                                        self_state_idx: None,
                                                                                        selector: None,
                                                                                        type_binding: Some(
                                                                                            TypeBinding {
                                                                                                inner: (),
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1156,
                                                                                                    end: 1158,
                                                                                                    as_str(): "+=",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1154,
                                                                                        end: 1160,
                                                                                        as_str(): "k += 1",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31845,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1154,
                                                                            end: 1160,
                                                                            as_str(): "k += 1",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 1154,
                                                                    end: 1160,
                                                                    as_str(): "k += 1",
                                                                },
                                                            },
                                                            TyAstNode {
                                                                content: Expression(
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
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1184,
                                                                                                    end: 1186,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1184,
                                                                                                    end: 1186,
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
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1184,
                                                                                                end: 1186,
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
                                                                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                                                                expression: FunctionApplication {
                                                                                                    call_path: CallPath {
                                                                                                        prefixes: [
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "core",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1180,
                                                                                                                    end: 1181,
                                                                                                                    as_str(): "%",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "ops",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1180,
                                                                                                                    end: 1181,
                                                                                                                    as_str(): "%",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ],
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "modulo",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1180,
                                                                                                                end: 1181,
                                                                                                                as_str(): "%",
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
                                                                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                    ),
                                                                                                                    start: 2008,
                                                                                                                    end: 2012,
                                                                                                                    as_str(): "self",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            TyExpression {
                                                                                                                expression: VariableExpression {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 512,
                                                                                                                            end: 513,
                                                                                                                            as_str(): "k",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1178,
                                                                                                                        end: 1179,
                                                                                                                        as_str(): "k",
                                                                                                                    },
                                                                                                                    mutability: Mutable,
                                                                                                                },
                                                                                                                return_type: TypeId(
                                                                                                                    21,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1178,
                                                                                                                    end: 1179,
                                                                                                                    as_str(): "k",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        (
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                    ),
                                                                                                                    start: 2014,
                                                                                                                    end: 2019,
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
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1182,
                                                                                                                    end: 1183,
                                                                                                                    as_str(): "2",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ],
                                                                                                    function_decl_id: DeclId(
                                                                                                        13348,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                                                            path: Some(
                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                            ),
                                                                                                            start: 1998,
                                                                                                            end: 2137,
                                                                                                            as_str(): "fn modulo(self, other: Self) -> Self {\n        asm(r1: self, r2: other, r3) {\n            mod r3 r1 r2;\n            r3: u64\n        }\n    }",
                                                                                                        },
                                                                                                    ),
                                                                                                    self_state_idx: None,
                                                                                                    selector: None,
                                                                                                    type_binding: Some(
                                                                                                        TypeBinding {
                                                                                                            inner: (),
                                                                                                            type_arguments: [],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1180,
                                                                                                                end: 1181,
                                                                                                                as_str(): "%",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1178,
                                                                                                    end: 1183,
                                                                                                    as_str(): "k % 2",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        (
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                                                                        0,
                                                                                                    ),
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1187,
                                                                                                    end: 1188,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    function_decl_id: DeclId(
                                                                                        13349,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1184,
                                                                                                end: 1186,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1178,
                                                                                    end: 1188,
                                                                                    as_str(): "k % 2 == 0",
                                                                                },
                                                                            },
                                                                            then: TyExpression {
                                                                                expression: CodeBlock(
                                                                                    TyCodeBlock {
                                                                                        contents: [
                                                                                            TyAstNode {
                                                                                                content: Expression(
                                                                                                    TyExpression {
                                                                                                        expression: Continue,
                                                                                                        return_type: TypeId(
                                                                                                            31855,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1207,
                                                                                                            end: 1215,
                                                                                                            as_str(): "continue",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1207,
                                                                                                    end: 1215,
                                                                                                    as_str(): "continue",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    31857,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1189,
                                                                                    end: 1230,
                                                                                    as_str(): "{\n                continue;\n            }",
                                                                                },
                                                                            },
                                                                            else: None,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31860,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1175,
                                                                            end: 1230,
                                                                            as_str(): "if k % 2 == 0 {\n                continue;\n            }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 1175,
                                                                    end: 1230,
                                                                    as_str(): "if k % 2 == 0 {\n                continue;\n            }",
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
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1244,
                                                                                        end: 1248,
                                                                                        as_str(): "sum1",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                lhs_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                lhs_indices: [],
                                                                                rhs: TyExpression {
                                                                                    expression: FunctionApplication {
                                                                                        call_path: CallPath {
                                                                                            prefixes: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "core",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1249,
                                                                                                        end: 1251,
                                                                                                        as_str(): "*=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1249,
                                                                                                        end: 1251,
                                                                                                        as_str(): "*=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "multiply",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1249,
                                                                                                    end: 1251,
                                                                                                    as_str(): "*=",
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
                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 1068,
                                                                                                        end: 1072,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: VariableExpression {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 550,
                                                                                                                end: 554,
                                                                                                                as_str(): "sum1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1244,
                                                                                                            end: 1248,
                                                                                                            as_str(): "sum1",
                                                                                                        },
                                                                                                        mutability: Mutable,
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1244,
                                                                                                        end: 1248,
                                                                                                        as_str(): "sum1",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 1074,
                                                                                                        end: 1079,
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
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1252,
                                                                                                        end: 1253,
                                                                                                        as_str(): "2",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            13350,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                ),
                                                                                                start: 1056,
                                                                                                end: 1129,
                                                                                                as_str(): "fn multiply(self, other: Self) -> Self {\n        __mul(self, other)\n    }",
                                                                                            },
                                                                                        ),
                                                                                        self_state_idx: None,
                                                                                        selector: None,
                                                                                        type_binding: Some(
                                                                                            TypeBinding {
                                                                                                inner: (),
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1249,
                                                                                                    end: 1251,
                                                                                                    as_str(): "*=",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1244,
                                                                                        end: 1253,
                                                                                        as_str(): "sum1 *= 2",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31867,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1244,
                                                                            end: 1253,
                                                                            as_str(): "sum1 *= 2",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 1244,
                                                                    end: 1253,
                                                                    as_str(): "sum1 *= 2",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                },
                                                return_type: TypeId(
                                                    31869,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 1037,
                                                    end: 1264,
                                                    as_str(): "while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1037,
                                            end: 1264,
                                            as_str(): "while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }",
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
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1273,
                                                                end: 1274,
                                                                as_str(): "i",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        lhs_type: TypeId(
                                                            21,
                                                        ),
                                                        lhs_indices: [],
                                                        rhs: TyExpression {
                                                            expression: FunctionApplication {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1275,
                                                                                end: 1277,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1275,
                                                                                end: 1277,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "add",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1275,
                                                                            end: 1277,
                                                                            as_str(): "+=",
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
                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 124,
                                                                                end: 128,
                                                                                as_str(): "self",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        TyExpression {
                                                                            expression: VariableExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 474,
                                                                                        end: 475,
                                                                                        as_str(): "i",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1273,
                                                                                    end: 1274,
                                                                                    as_str(): "i",
                                                                                },
                                                                                mutability: Mutable,
                                                                            },
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1273,
                                                                                end: 1274,
                                                                                as_str(): "i",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 130,
                                                                                end: 135,
                                                                                as_str(): "other",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
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
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1278,
                                                                                end: 1279,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                function_decl_id: DeclId(
                                                                    13351,
                                                                    Span {
                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 185,
                                                                        as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                    },
                                                                ),
                                                                self_state_idx: None,
                                                                selector: None,
                                                                type_binding: Some(
                                                                    TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1275,
                                                                            end: 1277,
                                                                            as_str(): "+=",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1273,
                                                                end: 1279,
                                                                as_str(): "i += 1",
                                                            },
                                                        },
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    31876,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 1273,
                                                    end: 1279,
                                                    as_str(): "i += 1",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1273,
                                            end: 1279,
                                            as_str(): "i += 1",
                                        },
                                    },
                                    TyAstNode {
                                        content: Expression(
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
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1299,
                                                                            end: 1301,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1299,
                                                                            end: 1301,
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
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1299,
                                                                        end: 1301,
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
                                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                        expression: FunctionApplication {
                                                                            call_path: CallPath {
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "core",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 1295,
                                                                                            end: 1296,
                                                                                            as_str(): "%",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 1295,
                                                                                            end: 1296,
                                                                                            as_str(): "%",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "modulo",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1295,
                                                                                        end: 1296,
                                                                                        as_str(): "%",
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
                                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 2008,
                                                                                            end: 2012,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 474,
                                                                                                    end: 475,
                                                                                                    as_str(): "i",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1293,
                                                                                                end: 1294,
                                                                                                as_str(): "i",
                                                                                            },
                                                                                            mutability: Mutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 1293,
                                                                                            end: 1294,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1357e0a80,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 2014,
                                                                                            end: 2019,
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
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 1297,
                                                                                            end: 1298,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            function_decl_id: DeclId(
                                                                                13352,
                                                                                Span {
                                                                                    src (ptr): 0x00007fb1357e0a80,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 1998,
                                                                                    end: 2137,
                                                                                    as_str(): "fn modulo(self, other: Self) -> Self {\n        asm(r1: self, r2: other, r3) {\n            mod r3 r1 r2;\n            r3: u64\n        }\n    }",
                                                                                },
                                                                            ),
                                                                            self_state_idx: None,
                                                                            selector: None,
                                                                            type_binding: Some(
                                                                                TypeBinding {
                                                                                    inner: (),
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1295,
                                                                                        end: 1296,
                                                                                        as_str(): "%",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1293,
                                                                            end: 1298,
                                                                            as_str(): "i % 2",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1302,
                                                                            end: 1303,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13353,
                                                                Span {
                                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1299,
                                                                        end: 1301,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1293,
                                                            end: 1303,
                                                            as_str(): "i % 2 == 0",
                                                        },
                                                    },
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: Expression(
                                                                            TyExpression {
                                                                                expression: Continue,
                                                                                return_type: TypeId(
                                                                                    31886,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1318,
                                                                                    end: 1326,
                                                                                    as_str(): "continue",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1318,
                                                                            end: 1326,
                                                                            as_str(): "continue",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31888,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1304,
                                                            end: 1337,
                                                            as_str(): "{\n            continue;\n        }",
                                                        },
                                                    },
                                                    else: None,
                                                },
                                                return_type: TypeId(
                                                    31891,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 1290,
                                                    end: 1337,
                                                    as_str(): "if i % 2 == 0 {\n            continue;\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1290,
                                            end: 1337,
                                            as_str(): "if i % 2 == 0 {\n            continue;\n        }",
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
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1347,
                                                                end: 1351,
                                                                as_str(): "sum1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        lhs_type: TypeId(
                                                            21,
                                                        ),
                                                        lhs_indices: [],
                                                        rhs: TyExpression {
                                                            expression: FunctionApplication {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1352,
                                                                                end: 1354,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1352,
                                                                                end: 1354,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "add",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1352,
                                                                            end: 1354,
                                                                            as_str(): "+=",
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
                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 124,
                                                                                end: 128,
                                                                                as_str(): "self",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        TyExpression {
                                                                            expression: VariableExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 550,
                                                                                        end: 554,
                                                                                        as_str(): "sum1",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1347,
                                                                                    end: 1351,
                                                                                    as_str(): "sum1",
                                                                                },
                                                                                mutability: Mutable,
                                                                            },
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1347,
                                                                                end: 1351,
                                                                                as_str(): "sum1",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 130,
                                                                                end: 135,
                                                                                as_str(): "other",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
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
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1355,
                                                                                end: 1356,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                function_decl_id: DeclId(
                                                                    13354,
                                                                    Span {
                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 185,
                                                                        as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                    },
                                                                ),
                                                                self_state_idx: None,
                                                                selector: None,
                                                                type_binding: Some(
                                                                    TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1352,
                                                                            end: 1354,
                                                                            as_str(): "+=",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1347,
                                                                end: 1356,
                                                                as_str(): "sum1 += 1",
                                                            },
                                                        },
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    31898,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 1347,
                                                    end: 1356,
                                                    as_str(): "sum1 += 1",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1347,
                                            end: 1356,
                                            as_str(): "sum1 += 1",
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
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1366,
                                                                end: 1370,
                                                                as_str(): "sum2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        lhs_type: TypeId(
                                                            21,
                                                        ),
                                                        lhs_indices: [],
                                                        rhs: TyExpression {
                                                            expression: FunctionApplication {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1371,
                                                                                end: 1373,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1371,
                                                                                end: 1373,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "add",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1371,
                                                                            end: 1373,
                                                                            as_str(): "+=",
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
                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 124,
                                                                                end: 128,
                                                                                as_str(): "self",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        TyExpression {
                                                                            expression: VariableExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 572,
                                                                                        end: 576,
                                                                                        as_str(): "sum2",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1366,
                                                                                    end: 1370,
                                                                                    as_str(): "sum2",
                                                                                },
                                                                                mutability: Mutable,
                                                                            },
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1366,
                                                                                end: 1370,
                                                                                as_str(): "sum2",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1357e0a80,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 130,
                                                                                end: 135,
                                                                                as_str(): "other",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
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
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1374,
                                                                                end: 1375,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                function_decl_id: DeclId(
                                                                    13355,
                                                                    Span {
                                                                        src (ptr): 0x00007fb1357e0a80,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 185,
                                                                        as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                    },
                                                                ),
                                                                self_state_idx: None,
                                                                selector: None,
                                                                type_binding: Some(
                                                                    TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1371,
                                                                            end: 1373,
                                                                            as_str(): "+=",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1366,
                                                                end: 1375,
                                                                as_str(): "sum2 += 1",
                                                            },
                                                        },
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    31905,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 1366,
                                                    end: 1375,
                                                    as_str(): "sum2 += 1",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1366,
                                            end: 1375,
                                            as_str(): "sum2 += 1",
                                        },
                                    },
                                ],
                            },
                        },
                        return_type: TypeId(
                            31907,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 586,
                            end: 1382,
                            as_str(): "while true {\n        if i >= N {\n            break;\n        }\n        while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }\n\n        while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }\n        i += 1;\n\n        if i % 2 == 0 {\n            continue;\n        }\n\n        sum1 += 1;\n        sum2 += 1;\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 586,
                    end: 1382,
                    as_str(): "while true {\n        if i >= N {\n            break;\n        }\n        while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }\n\n        while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }\n        i += 1;\n\n        if i % 2 == 0 {\n            continue;\n        }\n\n        sum1 += 1;\n        sum2 += 1;\n    }",
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
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 1388,
                                        end: 1394,
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
                                            src (ptr): 0x00007fb133074f40,
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
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1400,
                                                            end: 1402,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1400,
                                                            end: 1402,
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
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 1400,
                                                        end: 1402,
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
                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 550,
                                                                    end: 554,
                                                                    as_str(): "sum1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1395,
                                                                end: 1399,
                                                                as_str(): "sum1",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1395,
                                                            end: 1399,
                                                            as_str(): "sum1",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                3072,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1403,
                                                            end: 1407,
                                                            as_str(): "3072",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13357,
                                                Span {
                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 1400,
                                                        end: 1402,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1395,
                                            end: 1407,
                                            as_str(): "sum1 == 3072",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13358,
                                Span {
                                    src (ptr): 0x00007fb133074f40,
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
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 1388,
                                        end: 1394,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31913,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 1388,
                            end: 1408,
                            as_str(): "assert(sum1 == 3072)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 1388,
                    end: 1408,
                    as_str(): "assert(sum1 == 3072)",
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
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 1414,
                                        end: 1420,
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
                                            src (ptr): 0x00007fb133074f40,
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
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1426,
                                                            end: 1428,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1426,
                                                            end: 1428,
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
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 1426,
                                                        end: 1428,
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
                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 572,
                                                                    end: 576,
                                                                    as_str(): "sum2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1421,
                                                                end: 1425,
                                                                as_str(): "sum2",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1421,
                                                            end: 1425,
                                                            as_str(): "sum2",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb1357e0a80,
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
                                                                5,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1429,
                                                            end: 1430,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13360,
                                                Span {
                                                    src (ptr): 0x00007fb1357e0a80,
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
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 1426,
                                                        end: 1428,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1421,
                                            end: 1430,
                                            as_str(): "sum2 == 5",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13361,
                                Span {
                                    src (ptr): 0x00007fb133074f40,
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
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 1414,
                                        end: 1420,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31919,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 1414,
                            end: 1431,
                            as_str(): "assert(sum2 == 5)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 1414,
                    end: 1431,
                    as_str(): "assert(sum2 == 5)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb12b9f78e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
        ),
        start: 431,
        end: 1434,
        as_str(): "fn break_and_continue_test() {\n    let mut i = 0;\n    let mut j = 0;\n    let mut k = 0;\n    let mut n = 0;\n    let mut sum1 = 0;\n    let mut sum2 = 0;\n    while true {\n        if i >= N {\n            break;\n        }\n        while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }\n\n        while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }\n        i += 1;\n\n        if i % 2 == 0 {\n            continue;\n        }\n\n        sum1 += 1;\n        sum2 += 1;\n    }\n\n    assert(sum1 == 3072);\n    assert(sum2 == 5);\n}",
    },
    attributes: {},
    return_type: TypeId(
        31716,
    ),
    initial_return_type: TypeId(
        31715,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb12b9f78e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
        ),
        start: 431,
        end: 459,
        as_str(): "fn break_and_continue_test()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 1439,
            end: 1443,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
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
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 1460,
                                        end: 1477,
                                        as_str(): "simple_break_test",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13364,
                                Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 66,
                                    end: 218,
                                    as_str(): "fn simple_break_test() {\n    let mut i = 0;\n    while true {\n        if i >= N {\n            break;\n        }\n        i += 1\n    }\n    assert(i == N);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 1460,
                                        end: 1477,
                                        as_str(): "simple_break_test",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31923,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 1460,
                            end: 1479,
                            as_str(): "simple_break_test()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 1460,
                    end: 1479,
                    as_str(): "simple_break_test()",
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
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 1485,
                                        end: 1505,
                                        as_str(): "simple_continue_test",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13366,
                                Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 220,
                                    end: 429,
                                    as_str(): "fn simple_continue_test() {\n    let mut i = 0;\n    let mut sum = 0;\n    while i < N {\n        i += 1;\n        if i % 2 == 0 {\n            continue;\n        }\n        sum += 1;\n    }\n    assert(sum == N / 2);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 1485,
                                        end: 1505,
                                        as_str(): "simple_continue_test",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31925,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 1485,
                            end: 1507,
                            as_str(): "simple_continue_test()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 1485,
                    end: 1507,
                    as_str(): "simple_continue_test()",
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
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 1513,
                                        end: 1536,
                                        as_str(): "break_and_continue_test",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13368,
                                Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 431,
                                    end: 1434,
                                    as_str(): "fn break_and_continue_test() {\n    let mut i = 0;\n    let mut j = 0;\n    let mut k = 0;\n    let mut n = 0;\n    let mut sum1 = 0;\n    let mut sum2 = 0;\n    while true {\n        if i >= N {\n            break;\n        }\n        while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }\n\n        while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }\n        i += 1;\n\n        if i % 2 == 0 {\n            continue;\n        }\n\n        sum1 += 1;\n        sum2 += 1;\n    }\n\n    assert(sum1 == 3072);\n    assert(sum2 == 5);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 1513,
                                        end: 1536,
                                        as_str(): "break_and_continue_test",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31927,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 1513,
                            end: 1538,
                            as_str(): "break_and_continue_test()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 1513,
                    end: 1538,
                    as_str(): "break_and_continue_test()",
                },
            },
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
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 1545,
                            end: 1549,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12b9f78e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                    ),
                    start: 1545,
                    end: 1549,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb12b9f78e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
        ),
        start: 1436,
        end: 1551,
        as_str(): "fn main() -> bool {\n    simple_break_test();\n    simple_continue_test();\n    break_and_continue_test();\n\n    true\n}",
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
        src (ptr): 0x00007fb12b9f78e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
        ),
        start: 1449,
        end: 1453,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

