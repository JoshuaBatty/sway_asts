
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0e931aa40,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 57,
                                        end: 63,
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 81,
                                                            end: 83,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 81,
                                                            end: 83,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 81,
                                                        end: 83,
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
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 69,
                                                                            end: 73,
                                                                            as_str(): "true",
                                                                        },
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 75,
                                                                            end: 79,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 64,
                                                                    end: 80,
                                                                    as_str(): "__eq(true, true)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 64,
                                                            end: 80,
                                                            as_str(): "__eq(true, true)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 90,
                                                                            end: 92,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 90,
                                                                            end: 92,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 90,
                                                                        end: 92,
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 85,
                                                                            end: 89,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 93,
                                                                            end: 97,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13315,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 90,
                                                                        end: 92,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 85,
                                                            end: 97,
                                                            as_str(): "true == true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13316,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 81,
                                                        end: 83,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 64,
                                            end: 98,
                                            as_str(): "__eq(true, true) == (true == true)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13317,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 57,
                                        end: 63,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31638,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 57,
                            end: 99,
                            as_str(): "assert(__eq(true, true) == (true == true))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 57,
                    end: 99,
                    as_str(): "assert(__eq(true, true) == (true == true))",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 103,
                                        end: 109,
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 128,
                                                            end: 130,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 128,
                                                            end: 130,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "neq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 128,
                                                        end: 130,
                                                        as_str(): "!=",
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
                                                            src (ptr): 0x00007fb0ee2424d0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2832,
                                                            end: 2836,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 115,
                                                                            end: 119,
                                                                            as_str(): "true",
                                                                        },
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 121,
                                                                            end: 126,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 110,
                                                                    end: 127,
                                                                    as_str(): "__eq(true, false)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 110,
                                                            end: 127,
                                                            as_str(): "__eq(true, false)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2838,
                                                            end: 2843,
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 137,
                                                                            end: 139,
                                                                            as_str(): "!=",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 137,
                                                                            end: 139,
                                                                            as_str(): "!=",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "neq",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 137,
                                                                        end: 139,
                                                                        as_str(): "!=",
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2832,
                                                                            end: 2836,
                                                                            as_str(): "self",
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 132,
                                                                            end: 136,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2838,
                                                                            end: 2843,
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 140,
                                                                            end: 145,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13319,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2825,
                                                                    end: 2897,
                                                                    as_str(): "fn neq(self, other: Self) -> bool {\n        (self.eq(other)).not()\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 137,
                                                                        end: 139,
                                                                        as_str(): "!=",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 132,
                                                            end: 145,
                                                            as_str(): "true != false",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13320,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2825,
                                                    end: 2897,
                                                    as_str(): "fn neq(self, other: Self) -> bool {\n        (self.eq(other)).not()\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 128,
                                                        end: 130,
                                                        as_str(): "!=",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 110,
                                            end: 146,
                                            as_str(): "__eq(true, false) != (true != false)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13321,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 103,
                                        end: 109,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31646,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 103,
                            end: 147,
                            as_str(): "assert(__eq(true, false) != (true != false))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 103,
                    end: 147,
                    as_str(): "assert(__eq(true, false) != (true != false))",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 151,
                                        end: 157,
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 177,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 177,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 175,
                                                        end: 177,
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
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 163,
                                                                            end: 167,
                                                                            as_str(): "true",
                                                                        },
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 169,
                                                                            end: 173,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 158,
                                                                    end: 174,
                                                                    as_str(): "__eq(true, true)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 158,
                                                            end: 174,
                                                            as_str(): "__eq(true, true)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 184,
                                                                            end: 186,
                                                                            as_str(): "!=",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 184,
                                                                            end: 186,
                                                                            as_str(): "!=",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "neq",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 184,
                                                                        end: 186,
                                                                        as_str(): "!=",
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2832,
                                                                            end: 2836,
                                                                            as_str(): "self",
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 179,
                                                                            end: 183,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2838,
                                                                            end: 2843,
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 187,
                                                                            end: 192,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13323,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2825,
                                                                    end: 2897,
                                                                    as_str(): "fn neq(self, other: Self) -> bool {\n        (self.eq(other)).not()\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 184,
                                                                        end: 186,
                                                                        as_str(): "!=",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 179,
                                                            end: 192,
                                                            as_str(): "true != false",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13324,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 175,
                                                        end: 177,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 158,
                                            end: 193,
                                            as_str(): "__eq(true, true) == (true != false)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13325,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 151,
                                        end: 157,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31654,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 151,
                            end: 194,
                            as_str(): "assert(__eq(true, true) == (true != false))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 151,
                    end: 194,
                    as_str(): "assert(__eq(true, true) == (true != false))",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 218,
                                                            end: 220,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 218,
                                                            end: 220,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 218,
                                                        end: 220,
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
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 211,
                                                                            end: 212,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                22,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31660,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 214,
                                                                            end: 216,
                                                                            as_str(): "22",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 206,
                                                                    end: 217,
                                                                    as_str(): "__eq(1, 22)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 206,
                                                            end: 217,
                                                            as_str(): "__eq(1, 22)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 224,
                                                                            end: 226,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 224,
                                                                            end: 226,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 224,
                                                                        end: 226,
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                        expression: Literal(
                                                                            U64(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 222,
                                                                            end: 223,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                                22,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 227,
                                                                            end: 229,
                                                                            as_str(): "22",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13327,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 224,
                                                                        end: 226,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 222,
                                                            end: 229,
                                                            as_str(): "1 == 22",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13328,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 218,
                                                        end: 220,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 206,
                                            end: 230,
                                            as_str(): "__eq(1, 22) == (1 == 22)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13329,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 199,
                                        end: 205,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31666,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 199,
                            end: 231,
                            as_str(): "assert(__eq(1, 22) == (1 == 22))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 199,
                    end: 231,
                    as_str(): "assert(__eq(1, 22) == (1 == 22))",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 235,
                                        end: 241,
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 253,
                                                            end: 255,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 253,
                                                            end: 255,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 253,
                                                        end: 255,
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
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 247,
                                                                            end: 248,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31672,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 250,
                                                                            end: 251,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 242,
                                                                    end: 252,
                                                                    as_str(): "__eq(1, 1)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 242,
                                                            end: 252,
                                                            as_str(): "__eq(1, 1)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 259,
                                                                            end: 261,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 259,
                                                                            end: 261,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 259,
                                                                        end: 261,
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                        expression: Literal(
                                                                            U64(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 257,
                                                                            end: 258,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 262,
                                                                            end: 263,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13331,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 259,
                                                                        end: 261,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 257,
                                                            end: 263,
                                                            as_str(): "1 == 1",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13332,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 253,
                                                        end: 255,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 242,
                                            end: 264,
                                            as_str(): "__eq(1, 1) == (1 == 1)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13333,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 235,
                                        end: 241,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31678,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 235,
                            end: 265,
                            as_str(): "assert(__eq(1, 1) == (1 == 1))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 235,
                    end: 265,
                    as_str(): "assert(__eq(1, 1) == (1 == 1))",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 274,
                                    end: 275,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U8(
                                        1,
                                    ),
                                ),
                                return_type: TypeId(
                                    31679,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 282,
                                    end: 283,
                                    as_str(): "1",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                50,
                            ),
                            type_ascription: TypeId(
                                50,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 277,
                                    end: 279,
                                    as_str(): "u8",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 270,
                    end: 284,
                    as_str(): "let a: u8 = 1;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 291,
                                    end: 292,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U8(
                                        22,
                                    ),
                                ),
                                return_type: TypeId(
                                    31680,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 299,
                                    end: 301,
                                    as_str(): "22",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                50,
                            ),
                            type_ascription: TypeId(
                                50,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 294,
                                    end: 296,
                                    as_str(): "u8",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 287,
                    end: 302,
                    as_str(): "let b: u8 = 22;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 309,
                                    end: 310,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U8(
                                        1,
                                    ),
                                ),
                                return_type: TypeId(
                                    31681,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 317,
                                    end: 318,
                                    as_str(): "1",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                50,
                            ),
                            type_ascription: TypeId(
                                50,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 312,
                                    end: 314,
                                    as_str(): "u8",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 305,
                    end: 319,
                    as_str(): "let c: u8 = 1;",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 322,
                                        end: 328,
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 340,
                                                            end: 342,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 340,
                                                            end: 342,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 340,
                                                        end: 342,
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
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 274,
                                                                                    end: 275,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 334,
                                                                                end: 335,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 334,
                                                                            end: 335,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 291,
                                                                                    end: 292,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 337,
                                                                                end: 338,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 337,
                                                                            end: 338,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 329,
                                                                    end: 339,
                                                                    as_str(): "__eq(a, b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 329,
                                                            end: 339,
                                                            as_str(): "__eq(a, b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 346,
                                                                            end: 348,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 346,
                                                                            end: 348,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 346,
                                                                        end: 348,
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 274,
                                                                                    end: 275,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 344,
                                                                                end: 345,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 344,
                                                                            end: 345,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 291,
                                                                                    end: 292,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 349,
                                                                                end: 350,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 349,
                                                                            end: 350,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13335,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 346,
                                                                        end: 348,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 344,
                                                            end: 350,
                                                            as_str(): "a == b",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13336,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 340,
                                                        end: 342,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 329,
                                            end: 351,
                                            as_str(): "__eq(a, b) == (a == b)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13337,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 322,
                                        end: 328,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31689,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 322,
                            end: 352,
                            as_str(): "assert(__eq(a, b) == (a == b))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 322,
                    end: 352,
                    as_str(): "assert(__eq(a, b) == (a == b))",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 356,
                                        end: 362,
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 374,
                                                            end: 376,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 374,
                                                            end: 376,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 374,
                                                        end: 376,
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
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 274,
                                                                                    end: 275,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 368,
                                                                                end: 369,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 368,
                                                                            end: 369,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 309,
                                                                                    end: 310,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 371,
                                                                                end: 372,
                                                                                as_str(): "c",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 371,
                                                                            end: 372,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 363,
                                                                    end: 373,
                                                                    as_str(): "__eq(a, c)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 363,
                                                            end: 373,
                                                            as_str(): "__eq(a, c)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 380,
                                                                            end: 382,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 380,
                                                                            end: 382,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 380,
                                                                        end: 382,
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 274,
                                                                                    end: 275,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 378,
                                                                                end: 379,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 378,
                                                                            end: 379,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 309,
                                                                                    end: 310,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 383,
                                                                                end: 384,
                                                                                as_str(): "c",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 383,
                                                                            end: 384,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13339,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 380,
                                                                        end: 382,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 378,
                                                            end: 384,
                                                            as_str(): "a == c",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13340,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 374,
                                                        end: 376,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 363,
                                            end: 385,
                                            as_str(): "__eq(a, c) == (a == c)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13341,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 356,
                                        end: 362,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31697,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 356,
                            end: 386,
                            as_str(): "assert(__eq(a, c) == (a == c))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 356,
                    end: 386,
                    as_str(): "assert(__eq(a, c) == (a == c))",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 395,
                                    end: 396,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U16(
                                        1,
                                    ),
                                ),
                                return_type: TypeId(
                                    31698,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 404,
                                    end: 405,
                                    as_str(): "1",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                42,
                            ),
                            type_ascription: TypeId(
                                42,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 398,
                                    end: 401,
                                    as_str(): "u16",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 391,
                    end: 406,
                    as_str(): "let a: u16 = 1;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 413,
                                    end: 414,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U16(
                                        22,
                                    ),
                                ),
                                return_type: TypeId(
                                    31699,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 422,
                                    end: 424,
                                    as_str(): "22",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                42,
                            ),
                            type_ascription: TypeId(
                                42,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 416,
                                    end: 419,
                                    as_str(): "u16",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 409,
                    end: 425,
                    as_str(): "let b: u16 = 22;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 432,
                                    end: 433,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U16(
                                        1,
                                    ),
                                ),
                                return_type: TypeId(
                                    31700,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 441,
                                    end: 442,
                                    as_str(): "1",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                42,
                            ),
                            type_ascription: TypeId(
                                42,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 435,
                                    end: 438,
                                    as_str(): "u16",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 428,
                    end: 443,
                    as_str(): "let c: u16 = 1;",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 446,
                                        end: 452,
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 464,
                                                            end: 466,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 464,
                                                            end: 466,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 464,
                                                        end: 466,
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
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 395,
                                                                                    end: 396,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 458,
                                                                                end: 459,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 458,
                                                                            end: 459,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 413,
                                                                                    end: 414,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 461,
                                                                                end: 462,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 461,
                                                                            end: 462,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 453,
                                                                    end: 463,
                                                                    as_str(): "__eq(a, b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 453,
                                                            end: 463,
                                                            as_str(): "__eq(a, b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 470,
                                                                            end: 472,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 470,
                                                                            end: 472,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 470,
                                                                        end: 472,
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3206,
                                                                            end: 3210,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 395,
                                                                                    end: 396,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 468,
                                                                                end: 469,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 468,
                                                                            end: 469,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3212,
                                                                            end: 3217,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 413,
                                                                                    end: 414,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 473,
                                                                                end: 474,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 473,
                                                                            end: 474,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13343,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 3200,
                                                                    end: 3266,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 470,
                                                                        end: 472,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 468,
                                                            end: 474,
                                                            as_str(): "a == b",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13344,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 464,
                                                        end: 466,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 453,
                                            end: 475,
                                            as_str(): "__eq(a, b) == (a == b)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13345,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 446,
                                        end: 452,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31708,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 446,
                            end: 476,
                            as_str(): "assert(__eq(a, b) == (a == b))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 446,
                    end: 476,
                    as_str(): "assert(__eq(a, b) == (a == b))",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 480,
                                        end: 486,
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 498,
                                                            end: 500,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 498,
                                                            end: 500,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 498,
                                                        end: 500,
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
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 395,
                                                                                    end: 396,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 492,
                                                                                end: 493,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 492,
                                                                            end: 493,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 432,
                                                                                    end: 433,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 495,
                                                                                end: 496,
                                                                                as_str(): "c",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 495,
                                                                            end: 496,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 487,
                                                                    end: 497,
                                                                    as_str(): "__eq(a, c)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 487,
                                                            end: 497,
                                                            as_str(): "__eq(a, c)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 504,
                                                                            end: 506,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 504,
                                                                            end: 506,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 504,
                                                                        end: 506,
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3206,
                                                                            end: 3210,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 395,
                                                                                    end: 396,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 502,
                                                                                end: 503,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 502,
                                                                            end: 503,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3212,
                                                                            end: 3217,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 432,
                                                                                    end: 433,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 507,
                                                                                end: 508,
                                                                                as_str(): "c",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 507,
                                                                            end: 508,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13347,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 3200,
                                                                    end: 3266,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 504,
                                                                        end: 506,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 502,
                                                            end: 508,
                                                            as_str(): "a == c",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13348,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 498,
                                                        end: 500,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 487,
                                            end: 509,
                                            as_str(): "__eq(a, c) == (a == c)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13349,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 480,
                                        end: 486,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31716,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 480,
                            end: 510,
                            as_str(): "assert(__eq(a, c) == (a == c))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 480,
                    end: 510,
                    as_str(): "assert(__eq(a, c) == (a == c))",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 519,
                                    end: 520,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U32(
                                        1,
                                    ),
                                ),
                                return_type: TypeId(
                                    31717,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 528,
                                    end: 529,
                                    as_str(): "1",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33,
                            ),
                            type_ascription: TypeId(
                                33,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 522,
                                    end: 525,
                                    as_str(): "u32",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 515,
                    end: 530,
                    as_str(): "let a: u32 = 1;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 537,
                                    end: 538,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U32(
                                        22,
                                    ),
                                ),
                                return_type: TypeId(
                                    31718,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 546,
                                    end: 548,
                                    as_str(): "22",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33,
                            ),
                            type_ascription: TypeId(
                                33,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 540,
                                    end: 543,
                                    as_str(): "u32",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 533,
                    end: 549,
                    as_str(): "let b: u32 = 22;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 556,
                                    end: 557,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U32(
                                        1,
                                    ),
                                ),
                                return_type: TypeId(
                                    31719,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 565,
                                    end: 566,
                                    as_str(): "1",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33,
                            ),
                            type_ascription: TypeId(
                                33,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 559,
                                    end: 562,
                                    as_str(): "u32",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 552,
                    end: 567,
                    as_str(): "let c: u32 = 1;",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 570,
                                        end: 576,
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 588,
                                                            end: 590,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 588,
                                                            end: 590,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 588,
                                                        end: 590,
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
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 519,
                                                                                    end: 520,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 582,
                                                                                end: 583,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 582,
                                                                            end: 583,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 537,
                                                                                    end: 538,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 585,
                                                                                end: 586,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 585,
                                                                            end: 586,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 577,
                                                                    end: 587,
                                                                    as_str(): "__eq(a, b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 577,
                                                            end: 587,
                                                            as_str(): "__eq(a, b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 594,
                                                                            end: 596,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 594,
                                                                            end: 596,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 594,
                                                                        end: 596,
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 519,
                                                                                    end: 520,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 592,
                                                                                end: 593,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 592,
                                                                            end: 593,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 537,
                                                                                    end: 538,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 597,
                                                                                end: 598,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 597,
                                                                            end: 598,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13351,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 594,
                                                                        end: 596,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 592,
                                                            end: 598,
                                                            as_str(): "a == b",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13352,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 588,
                                                        end: 590,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 577,
                                            end: 599,
                                            as_str(): "__eq(a, b) == (a == b)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13353,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 570,
                                        end: 576,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31727,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 570,
                            end: 600,
                            as_str(): "assert(__eq(a, b) == (a == b))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 570,
                    end: 600,
                    as_str(): "assert(__eq(a, b) == (a == b))",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 604,
                                        end: 610,
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 622,
                                                            end: 624,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 622,
                                                            end: 624,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 622,
                                                        end: 624,
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
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 519,
                                                                                    end: 520,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 616,
                                                                                end: 617,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 616,
                                                                            end: 617,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 556,
                                                                                    end: 557,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 619,
                                                                                end: 620,
                                                                                as_str(): "c",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 619,
                                                                            end: 620,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 611,
                                                                    end: 621,
                                                                    as_str(): "__eq(a, c)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 611,
                                                            end: 621,
                                                            as_str(): "__eq(a, c)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 519,
                                                                                    end: 520,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 626,
                                                                                end: 627,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 626,
                                                                            end: 627,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 556,
                                                                                    end: 557,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 631,
                                                                                end: 632,
                                                                                as_str(): "c",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 631,
                                                                            end: 632,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13355,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 626,
                                                            end: 632,
                                                            as_str(): "a == c",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13356,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 622,
                                                        end: 624,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 611,
                                            end: 633,
                                            as_str(): "__eq(a, c) == (a == c)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13357,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 604,
                                        end: 610,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31735,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 604,
                            end: 634,
                            as_str(): "assert(__eq(a, c) == (a == c))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 604,
                    end: 634,
                    as_str(): "assert(__eq(a, c) == (a == c))",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 643,
                                    end: 644,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        1,
                                    ),
                                ),
                                return_type: TypeId(
                                    31736,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 652,
                                    end: 653,
                                    as_str(): "1",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                21,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 646,
                                    end: 649,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 639,
                    end: 654,
                    as_str(): "let a: u64 = 1;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 661,
                                    end: 662,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        22,
                                    ),
                                ),
                                return_type: TypeId(
                                    31737,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 670,
                                    end: 672,
                                    as_str(): "22",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                21,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 664,
                                    end: 667,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 657,
                    end: 673,
                    as_str(): "let b: u64 = 22;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 680,
                                    end: 681,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        1,
                                    ),
                                ),
                                return_type: TypeId(
                                    31738,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 689,
                                    end: 690,
                                    as_str(): "1",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                21,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 683,
                                    end: 686,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 676,
                    end: 691,
                    as_str(): "let c: u64 = 1;",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 694,
                                        end: 700,
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 712,
                                                            end: 714,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 712,
                                                            end: 714,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 712,
                                                        end: 714,
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
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 643,
                                                                                    end: 644,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 706,
                                                                                end: 707,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 706,
                                                                            end: 707,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 661,
                                                                                    end: 662,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 709,
                                                                                end: 710,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 709,
                                                                            end: 710,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 701,
                                                                    end: 711,
                                                                    as_str(): "__eq(a, b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 701,
                                                            end: 711,
                                                            as_str(): "__eq(a, b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 718,
                                                                            end: 720,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 718,
                                                                            end: 720,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 718,
                                                                        end: 720,
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 643,
                                                                                    end: 644,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 716,
                                                                                end: 717,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 716,
                                                                            end: 717,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 661,
                                                                                    end: 662,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 721,
                                                                                end: 722,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 721,
                                                                            end: 722,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13359,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 718,
                                                                        end: 720,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 716,
                                                            end: 722,
                                                            as_str(): "a == b",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13360,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 712,
                                                        end: 714,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 701,
                                            end: 723,
                                            as_str(): "__eq(a, b) == (a == b)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13361,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 694,
                                        end: 700,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31746,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 694,
                            end: 724,
                            as_str(): "assert(__eq(a, b) == (a == b))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 694,
                    end: 724,
                    as_str(): "assert(__eq(a, b) == (a == b))",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 728,
                                        end: 734,
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
                                            src (ptr): 0x00007fb0f1e2d6d0,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 746,
                                                            end: 748,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 746,
                                                            end: 748,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 746,
                                                        end: 748,
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
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Eq,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 643,
                                                                                    end: 644,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 740,
                                                                                end: 741,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 740,
                                                                            end: 741,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 680,
                                                                                    end: 681,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 743,
                                                                                end: 744,
                                                                                as_str(): "c",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 743,
                                                                            end: 744,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 735,
                                                                    end: 745,
                                                                    as_str(): "__eq(a, c)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 735,
                                                            end: 745,
                                                            as_str(): "__eq(a, c)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 752,
                                                                            end: 754,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 752,
                                                                            end: 754,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 752,
                                                                        end: 754,
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
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 643,
                                                                                    end: 644,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 750,
                                                                                end: 751,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 750,
                                                                            end: 751,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee2424d0,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 680,
                                                                                    end: 681,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 755,
                                                                                end: 756,
                                                                                as_str(): "c",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 755,
                                                                            end: 756,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13363,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 752,
                                                                        end: 754,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 750,
                                                            end: 756,
                                                            as_str(): "a == c",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13364,
                                                Span {
                                                    src (ptr): 0x00007fb0ee2424d0,
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
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 746,
                                                        end: 748,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 735,
                                            end: 757,
                                            as_str(): "__eq(a, c) == (a == c)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13365,
                                Span {
                                    src (ptr): 0x00007fb0f1e2d6d0,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 728,
                                        end: 734,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31754,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 728,
                            end: 758,
                            as_str(): "assert(__eq(a, c) == (a == c))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 728,
                    end: 758,
                    as_str(): "assert(__eq(a, c) == (a == c))",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                2,
                            ),
                        ),
                        return_type: TypeId(
                            31755,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 763,
                            end: 764,
                            as_str(): "2",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e931aa40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                    ),
                    start: 763,
                    end: 764,
                    as_str(): "2",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb0e931aa40,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
        ),
        start: 35,
        end: 766,
        as_str(): "fn main() -> u64 {\n\n  assert(__eq(true, true) == (true == true));\n  assert(__eq(true, false) != (true != false));\n  assert(__eq(true, true) == (true != false));\n\n  assert(__eq(1, 22) == (1 == 22));\n  assert(__eq(1, 1) == (1 == 1));\n\n  let a: u8 = 1;\n  let b: u8 = 22;\n  let c: u8 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u16 = 1;\n  let b: u16 = 22;\n  let c: u16 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u32 = 1;\n  let b: u32 = 22;\n  let c: u32 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u64 = 1;\n  let b: u64 = 22;\n  let c: u64 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  2\n}",
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
        src (ptr): 0x00007fb0e931aa40,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
        ),
        start: 48,
        end: 51,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

