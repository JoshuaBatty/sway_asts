
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0503ea450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
            ),
            start: 38,
            end: 42,
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
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 66,
                                    end: 67,
                                    as_str(): "a",
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
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 70,
                                    end: 71,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31631,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 58,
                    end: 72,
                    as_str(): "let mut a = 0;",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 82,
                                        end: 83,
                                        as_str(): "a",
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 84,
                                                        end: 86,
                                                        as_str(): "+=",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "ops",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 84,
                                                        end: 86,
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
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 84,
                                                    end: 86,
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
                                                        src (ptr): 0x00007fe06dbec080,
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 66,
                                                                end: 67,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 82,
                                                            end: 83,
                                                            as_str(): "a",
                                                        },
                                                        mutability: Mutable,
                                                    },
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 82,
                                                        end: 83,
                                                        as_str(): "a",
                                                    },
                                                },
                                            ),
                                            (
                                                BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe06dbec080,
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
                                                            99,
                                                        ),
                                                    ),
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 87,
                                                        end: 89,
                                                        as_str(): "99",
                                                    },
                                                },
                                            ),
                                        ],
                                        function_decl_id: DeclId(
                                            13314,
                                            Span {
                                                src (ptr): 0x00007fe06dbec080,
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
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 84,
                                                    end: 86,
                                                    as_str(): "+=",
                                                },
                                            },
                                        ),
                                    },
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 82,
                                        end: 89,
                                        as_str(): "a += 99",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            31639,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 82,
                            end: 89,
                            as_str(): "a += 99",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 82,
                    end: 89,
                    as_str(): "a += 99",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 101,
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
                                            src (ptr): 0x00007fe06e257550,
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
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 104,
                                                            end: 106,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 104,
                                                            end: 106,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 104,
                                                        end: 106,
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
                                                            src (ptr): 0x00007fe06dbec080,
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 66,
                                                                    end: 67,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 102,
                                                                end: 103,
                                                                as_str(): "a",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 102,
                                                            end: 103,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe06dbec080,
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
                                                                99,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 107,
                                                            end: 109,
                                                            as_str(): "99",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13316,
                                                Span {
                                                    src (ptr): 0x00007fe06dbec080,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 104,
                                                        end: 106,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 102,
                                            end: 109,
                                            as_str(): "a == 99",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13317,
                                Span {
                                    src (ptr): 0x00007fe06e257550,
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 101,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31645,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 95,
                            end: 110,
                            as_str(): "assert(a == 99)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 95,
                    end: 110,
                    as_str(): "assert(a == 99)",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 117,
                                        end: 118,
                                        as_str(): "a",
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 119,
                                                        end: 121,
                                                        as_str(): "-=",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "ops",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 119,
                                                        end: 121,
                                                        as_str(): "-=",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            ],
                                            suffix: BaseIdent {
                                                name_override_opt: Some(
                                                    "subtract",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 119,
                                                    end: 121,
                                                    as_str(): "-=",
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
                                                        src (ptr): 0x00007fe06dbec080,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                        ),
                                                        start: 581,
                                                        end: 585,
                                                        as_str(): "self",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                TyExpression {
                                                    expression: VariableExpression {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 66,
                                                                end: 67,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 117,
                                                            end: 118,
                                                            as_str(): "a",
                                                        },
                                                        mutability: Mutable,
                                                    },
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 117,
                                                        end: 118,
                                                        as_str(): "a",
                                                    },
                                                },
                                            ),
                                            (
                                                BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe06dbec080,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                        ),
                                                        start: 587,
                                                        end: 592,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 122,
                                                        end: 123,
                                                        as_str(): "5",
                                                    },
                                                },
                                            ),
                                        ],
                                        function_decl_id: DeclId(
                                            13318,
                                            Span {
                                                src (ptr): 0x00007fe06dbec080,
                                                path: Some(
                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                ),
                                                start: 569,
                                                end: 642,
                                                as_str(): "fn subtract(self, other: Self) -> Self {\n        __sub(self, other)\n    }",
                                            },
                                        ),
                                        self_state_idx: None,
                                        selector: None,
                                        type_binding: Some(
                                            TypeBinding {
                                                inner: (),
                                                type_arguments: [],
                                                span: Span {
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 119,
                                                    end: 121,
                                                    as_str(): "-=",
                                                },
                                            },
                                        ),
                                    },
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 117,
                                        end: 123,
                                        as_str(): "a -= 5",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            31652,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 117,
                            end: 123,
                            as_str(): "a -= 5",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 117,
                    end: 123,
                    as_str(): "a -= 5",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 129,
                                        end: 135,
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
                                            src (ptr): 0x00007fe06e257550,
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
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 138,
                                                            end: 140,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 138,
                                                            end: 140,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 138,
                                                        end: 140,
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
                                                            src (ptr): 0x00007fe06dbec080,
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 66,
                                                                    end: 67,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 136,
                                                                end: 137,
                                                                as_str(): "a",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 136,
                                                            end: 137,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe06dbec080,
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
                                                                94,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 141,
                                                            end: 143,
                                                            as_str(): "94",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13320,
                                                Span {
                                                    src (ptr): 0x00007fe06dbec080,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 138,
                                                        end: 140,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 136,
                                            end: 143,
                                            as_str(): "a == 94",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13321,
                                Span {
                                    src (ptr): 0x00007fe06e257550,
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 129,
                                        end: 135,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31658,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 129,
                            end: 144,
                            as_str(): "assert(a == 94)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 129,
                    end: 144,
                    as_str(): "assert(a == 94)",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 151,
                                        end: 152,
                                        as_str(): "a",
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 153,
                                                        end: 155,
                                                        as_str(): "*=",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "ops",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 153,
                                                        end: 155,
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
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 153,
                                                    end: 155,
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
                                                        src (ptr): 0x00007fe06dbec080,
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 66,
                                                                end: 67,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 151,
                                                            end: 152,
                                                            as_str(): "a",
                                                        },
                                                        mutability: Mutable,
                                                    },
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 151,
                                                        end: 152,
                                                        as_str(): "a",
                                                    },
                                                },
                                            ),
                                            (
                                                BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe06dbec080,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 156,
                                                        end: 157,
                                                        as_str(): "2",
                                                    },
                                                },
                                            ),
                                        ],
                                        function_decl_id: DeclId(
                                            13322,
                                            Span {
                                                src (ptr): 0x00007fe06dbec080,
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
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 153,
                                                    end: 155,
                                                    as_str(): "*=",
                                                },
                                            },
                                        ),
                                    },
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 151,
                                        end: 157,
                                        as_str(): "a *= 2",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            31665,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 151,
                            end: 157,
                            as_str(): "a *= 2",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 151,
                    end: 157,
                    as_str(): "a *= 2",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 163,
                                        end: 169,
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
                                            src (ptr): 0x00007fe06e257550,
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
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 174,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 174,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 172,
                                                        end: 174,
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
                                                            src (ptr): 0x00007fe06dbec080,
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 66,
                                                                    end: 67,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 170,
                                                                end: 171,
                                                                as_str(): "a",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 170,
                                                            end: 171,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe06dbec080,
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
                                                                188,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 178,
                                                            as_str(): "188",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13324,
                                                Span {
                                                    src (ptr): 0x00007fe06dbec080,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 172,
                                                        end: 174,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 170,
                                            end: 178,
                                            as_str(): "a == 188",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13325,
                                Span {
                                    src (ptr): 0x00007fe06e257550,
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 163,
                                        end: 169,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31671,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 163,
                            end: 179,
                            as_str(): "assert(a == 188)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 163,
                    end: 179,
                    as_str(): "assert(a == 188)",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 186,
                                        end: 187,
                                        as_str(): "a",
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 188,
                                                        end: 190,
                                                        as_str(): "/=",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "ops",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 188,
                                                        end: 190,
                                                        as_str(): "/=",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            ],
                                            suffix: BaseIdent {
                                                name_override_opt: Some(
                                                    "divide",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 188,
                                                    end: 190,
                                                    as_str(): "/=",
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
                                                        src (ptr): 0x00007fe06dbec080,
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 66,
                                                                end: 67,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 186,
                                                            end: 187,
                                                            as_str(): "a",
                                                        },
                                                        mutability: Mutable,
                                                    },
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 186,
                                                        end: 187,
                                                        as_str(): "a",
                                                    },
                                                },
                                            ),
                                            (
                                                BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe06dbec080,
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
                                                            47,
                                                        ),
                                                    ),
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 191,
                                                        end: 193,
                                                        as_str(): "47",
                                                    },
                                                },
                                            ),
                                        ],
                                        function_decl_id: DeclId(
                                            13326,
                                            Span {
                                                src (ptr): 0x00007fe06dbec080,
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
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 188,
                                                    end: 190,
                                                    as_str(): "/=",
                                                },
                                            },
                                        ),
                                    },
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 186,
                                        end: 193,
                                        as_str(): "a /= 47",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            31678,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 186,
                            end: 193,
                            as_str(): "a /= 47",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 186,
                    end: 193,
                    as_str(): "a /= 47",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 199,
                                        end: 205,
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
                                            src (ptr): 0x00007fe06e257550,
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
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 208,
                                                            end: 210,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 208,
                                                            end: 210,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 208,
                                                        end: 210,
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
                                                            src (ptr): 0x00007fe06dbec080,
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 66,
                                                                    end: 67,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 206,
                                                                end: 207,
                                                                as_str(): "a",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 206,
                                                            end: 207,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe06dbec080,
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
                                                                4,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 211,
                                                            end: 212,
                                                            as_str(): "4",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13328,
                                                Span {
                                                    src (ptr): 0x00007fe06dbec080,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 208,
                                                        end: 210,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 206,
                                            end: 212,
                                            as_str(): "a == 4",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13329,
                                Span {
                                    src (ptr): 0x00007fe06e257550,
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 199,
                                        end: 205,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31684,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 199,
                            end: 213,
                            as_str(): "assert(a == 4)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 199,
                    end: 213,
                    as_str(): "assert(a == 4)",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 220,
                                        end: 221,
                                        as_str(): "a",
                                    },
                                    is_raw_ident: false,
                                },
                                lhs_type: TypeId(
                                    21,
                                ),
                                lhs_indices: [],
                                rhs: TyExpression {
                                    expression: Literal(
                                        U64(
                                            999,
                                        ),
                                    ),
                                    return_type: TypeId(
                                        31687,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 224,
                                        end: 227,
                                        as_str(): "999",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            31689,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 220,
                            end: 227,
                            as_str(): "a = 999",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 220,
                    end: 227,
                    as_str(): "a = 999",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 234,
                                        end: 235,
                                        as_str(): "a",
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 236,
                                                        end: 239,
                                                        as_str(): ">>=",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "ops",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 236,
                                                        end: 239,
                                                        as_str(): ">>=",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            ],
                                            suffix: BaseIdent {
                                                name_override_opt: Some(
                                                    "rsh",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 236,
                                                    end: 239,
                                                    as_str(): ">>=",
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
                                                        src (ptr): 0x00007fe06dbec080,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                        ),
                                                        start: 11608,
                                                        end: 11612,
                                                        as_str(): "self",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                TyExpression {
                                                    expression: VariableExpression {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 66,
                                                                end: 67,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 234,
                                                            end: 235,
                                                            as_str(): "a",
                                                        },
                                                        mutability: Mutable,
                                                    },
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 234,
                                                        end: 235,
                                                        as_str(): "a",
                                                    },
                                                },
                                            ),
                                            (
                                                BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe06dbec080,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                        ),
                                                        start: 11614,
                                                        end: 11619,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 240,
                                                        end: 241,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ),
                                        ],
                                        function_decl_id: DeclId(
                                            13330,
                                            Span {
                                                src (ptr): 0x00007fe06dbec080,
                                                path: Some(
                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                ),
                                                start: 11601,
                                                end: 11736,
                                                as_str(): "fn rsh(self, other: u64) -> Self {\n        asm(r1: self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }",
                                            },
                                        ),
                                        self_state_idx: None,
                                        selector: None,
                                        type_binding: Some(
                                            TypeBinding {
                                                inner: (),
                                                type_arguments: [],
                                                span: Span {
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 236,
                                                    end: 239,
                                                    as_str(): ">>=",
                                                },
                                            },
                                        ),
                                    },
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 234,
                                        end: 241,
                                        as_str(): "a >>= 1",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            31696,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 234,
                            end: 241,
                            as_str(): "a >>= 1",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 234,
                    end: 241,
                    as_str(): "a >>= 1",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 247,
                                        end: 253,
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
                                            src (ptr): 0x00007fe06e257550,
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
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 256,
                                                            end: 258,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 256,
                                                            end: 258,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 256,
                                                        end: 258,
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
                                                            src (ptr): 0x00007fe06dbec080,
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 66,
                                                                    end: 67,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 254,
                                                                end: 255,
                                                                as_str(): "a",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 254,
                                                            end: 255,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe06dbec080,
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
                                                                499,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 259,
                                                            end: 262,
                                                            as_str(): "499",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13332,
                                                Span {
                                                    src (ptr): 0x00007fe06dbec080,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 256,
                                                        end: 258,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 254,
                                            end: 262,
                                            as_str(): "a == 499",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13333,
                                Span {
                                    src (ptr): 0x00007fe06e257550,
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 247,
                                        end: 253,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31702,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 247,
                            end: 263,
                            as_str(): "assert(a == 499)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 247,
                    end: 263,
                    as_str(): "assert(a == 499)",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 270,
                                        end: 271,
                                        as_str(): "a",
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 272,
                                                        end: 275,
                                                        as_str(): "<<=",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "ops",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 272,
                                                        end: 275,
                                                        as_str(): "<<=",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            ],
                                            suffix: BaseIdent {
                                                name_override_opt: Some(
                                                    "lsh",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 272,
                                                    end: 275,
                                                    as_str(): "<<=",
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
                                                        src (ptr): 0x00007fe06dbec080,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                        ),
                                                        start: 11468,
                                                        end: 11472,
                                                        as_str(): "self",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                TyExpression {
                                                    expression: VariableExpression {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 66,
                                                                end: 67,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 270,
                                                            end: 271,
                                                            as_str(): "a",
                                                        },
                                                        mutability: Mutable,
                                                    },
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 270,
                                                        end: 271,
                                                        as_str(): "a",
                                                    },
                                                },
                                            ),
                                            (
                                                BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe06dbec080,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                        ),
                                                        start: 11474,
                                                        end: 11479,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 276,
                                                        end: 277,
                                                        as_str(): "2",
                                                    },
                                                },
                                            ),
                                        ],
                                        function_decl_id: DeclId(
                                            13334,
                                            Span {
                                                src (ptr): 0x00007fe06dbec080,
                                                path: Some(
                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                ),
                                                start: 11461,
                                                end: 11596,
                                                as_str(): "fn lsh(self, other: u64) -> Self {\n        asm(r1: self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }",
                                            },
                                        ),
                                        self_state_idx: None,
                                        selector: None,
                                        type_binding: Some(
                                            TypeBinding {
                                                inner: (),
                                                type_arguments: [],
                                                span: Span {
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 272,
                                                    end: 275,
                                                    as_str(): "<<=",
                                                },
                                            },
                                        ),
                                    },
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 270,
                                        end: 277,
                                        as_str(): "a <<= 2",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            31709,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 270,
                            end: 277,
                            as_str(): "a <<= 2",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 270,
                    end: 277,
                    as_str(): "a <<= 2",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 283,
                                        end: 289,
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
                                            src (ptr): 0x00007fe06e257550,
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
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 292,
                                                            end: 294,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 292,
                                                            end: 294,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 292,
                                                        end: 294,
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
                                                            src (ptr): 0x00007fe06dbec080,
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 66,
                                                                    end: 67,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 290,
                                                                end: 291,
                                                                as_str(): "a",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 290,
                                                            end: 291,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe06dbec080,
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
                                                                1996,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 295,
                                                            end: 299,
                                                            as_str(): "1996",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13336,
                                                Span {
                                                    src (ptr): 0x00007fe06dbec080,
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 292,
                                                        end: 294,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 290,
                                            end: 299,
                                            as_str(): "a == 1996",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13337,
                                Span {
                                    src (ptr): 0x00007fe06e257550,
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 283,
                                        end: 289,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31715,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 283,
                            end: 300,
                            as_str(): "assert(a == 1996)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 283,
                    end: 300,
                    as_str(): "assert(a == 1996)",
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
                            31716,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 307,
                            end: 308,
                            as_str(): "1",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0503ea450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                    ),
                    start: 307,
                    end: 308,
                    as_str(): "1",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0503ea450,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
        ),
        start: 35,
        end: 310,
        as_str(): "fn main() -> u64 {\n    let mut a = 0;\n    \n    a += 99;\n    assert(a == 99);\n\n    a -= 5;\n    assert(a == 94);\n\n    a *= 2;\n    assert(a == 188);\n\n    a /= 47;\n    assert(a == 4);\n\n    a = 999;\n\n    a >>= 1;\n    assert(a == 499);\n\n    a <<= 2;\n    assert(a == 1996);\n\n    1\n}",
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
        src (ptr): 0x00007fe0503ea450,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
        ),
        start: 48,
        end: 51,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

