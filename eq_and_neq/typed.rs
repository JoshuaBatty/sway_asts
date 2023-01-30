



TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0e6588390,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
            ),
            start: 120,
            end: 124,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 164,
                                        end: 170,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                        expression: Literal(
                                                            U8(
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 171,
                                                            end: 174,
                                                            as_str(): "1u8",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 178,
                                                            end: 181,
                                                            as_str(): "1u8",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13315,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
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
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 171,
                                            end: 181,
                                            as_str(): "1u8 == 1u8",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13316,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 164,
                                        end: 170,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31635,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 164,
                            end: 182,
                            as_str(): "assert(1u8 == 1u8)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 164,
                    end: 182,
                    as_str(): "assert(1u8 == 1u8)",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 188,
                                        end: 194,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 199,
                                                            end: 201,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 199,
                                                            end: 201,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 199,
                                                        end: 201,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            U8(
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 195,
                                                            end: 198,
                                                            as_str(): "1u8",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            U8(
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 202,
                                                            end: 205,
                                                            as_str(): "2u8",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13318,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 199,
                                                        end: 201,
                                                        as_str(): "!=",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 195,
                                            end: 205,
                                            as_str(): "1u8 != 2u8",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13319,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 188,
                                        end: 194,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31640,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 188,
                            end: 206,
                            as_str(): "assert(1u8 != 2u8)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 188,
                    end: 206,
                    as_str(): "assert(1u8 != 2u8)",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 213,
                                        end: 219,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 225,
                                                            end: 227,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 225,
                                                            end: 227,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 225,
                                                        end: 227,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                        expression: Literal(
                                                            U16(
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 220,
                                                            end: 224,
                                                            as_str(): "1u16",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                        expression: Literal(
                                                            U16(
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 228,
                                                            end: 232,
                                                            as_str(): "1u16",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13321,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 225,
                                                        end: 227,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 220,
                                            end: 232,
                                            as_str(): "1u16 == 1u16",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13322,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 213,
                                        end: 219,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31645,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 213,
                            end: 233,
                            as_str(): "assert(1u16 == 1u16)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 213,
                    end: 233,
                    as_str(): "assert(1u16 == 1u16)",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 239,
                                        end: 245,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 251,
                                                            end: 253,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 251,
                                                            end: 253,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 251,
                                                        end: 253,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            U16(
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 246,
                                                            end: 250,
                                                            as_str(): "1u16",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            U16(
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 254,
                                                            end: 258,
                                                            as_str(): "2u16",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13324,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 251,
                                                        end: 253,
                                                        as_str(): "!=",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 246,
                                            end: 258,
                                            as_str(): "1u16 != 2u16",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13325,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 239,
                                        end: 245,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31650,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 239,
                            end: 259,
                            as_str(): "assert(1u16 != 2u16)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 239,
                    end: 259,
                    as_str(): "assert(1u16 != 2u16)",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 266,
                                        end: 272,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 278,
                                                            end: 280,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 278,
                                                            end: 280,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 278,
                                                        end: 280,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                        expression: Literal(
                                                            U32(
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 273,
                                                            end: 277,
                                                            as_str(): "1u32",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 281,
                                                            end: 285,
                                                            as_str(): "1u32",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13327,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 278,
                                                        end: 280,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 273,
                                            end: 285,
                                            as_str(): "1u32 == 1u32",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13328,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 266,
                                        end: 272,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31655,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 266,
                            end: 286,
                            as_str(): "assert(1u32 == 1u32)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 266,
                    end: 286,
                    as_str(): "assert(1u32 == 1u32)",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 292,
                                        end: 298,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 304,
                                                            end: 306,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 304,
                                                            end: 306,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 304,
                                                        end: 306,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            U32(
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 299,
                                                            end: 303,
                                                            as_str(): "1u32",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            U32(
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 307,
                                                            end: 311,
                                                            as_str(): "2u32",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13330,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 304,
                                                        end: 306,
                                                        as_str(): "!=",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 299,
                                            end: 311,
                                            as_str(): "1u32 != 2u32",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13331,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 292,
                                        end: 298,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31660,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 292,
                            end: 312,
                            as_str(): "assert(1u32 != 2u32)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 292,
                    end: 312,
                    as_str(): "assert(1u32 != 2u32)",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 319,
                                        end: 325,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 331,
                                                            end: 333,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 331,
                                                            end: 333,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 331,
                                                        end: 333,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 326,
                                                            end: 330,
                                                            as_str(): "1u64",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 334,
                                                            end: 338,
                                                            as_str(): "1u64",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13333,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 331,
                                                        end: 333,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 326,
                                            end: 338,
                                            as_str(): "1u64 == 1u64",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13334,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 319,
                                        end: 325,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31665,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 319,
                            end: 339,
                            as_str(): "assert(1u64 == 1u64)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 319,
                    end: 339,
                    as_str(): "assert(1u64 == 1u64)",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 345,
                                        end: 351,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 357,
                                                            end: 359,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 357,
                                                            end: 359,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 357,
                                                        end: 359,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            U64(
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 352,
                                                            end: 356,
                                                            as_str(): "1u64",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            U64(
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 360,
                                                            end: 364,
                                                            as_str(): "2u64",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13336,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 357,
                                                        end: 359,
                                                        as_str(): "!=",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 352,
                                            end: 364,
                                            as_str(): "1u64 != 2u64",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13337,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 345,
                                        end: 351,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31670,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 345,
                            end: 365,
                            as_str(): "assert(1u64 != 2u64)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 345,
                    end: 365,
                    as_str(): "assert(1u64 != 2u64)",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 372,
                                        end: 378,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 384,
                                                            end: 386,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 384,
                                                            end: 386,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 384,
                                                        end: 386,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 379,
                                                            end: 383,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 387,
                                                            end: 391,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13339,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 384,
                                                        end: 386,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 379,
                                            end: 391,
                                            as_str(): "true == true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13340,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 372,
                                        end: 378,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31675,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 372,
                            end: 392,
                            as_str(): "assert(true == true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 372,
                    end: 392,
                    as_str(): "assert(true == true)",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 398,
                                        end: 404,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 410,
                                                            end: 412,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 410,
                                                            end: 412,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 410,
                                                        end: 412,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 405,
                                                            end: 409,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 413,
                                                            end: 418,
                                                            as_str(): "false",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13342,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 410,
                                                        end: 412,
                                                        as_str(): "!=",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 405,
                                            end: 418,
                                            as_str(): "true != false",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13343,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 398,
                                        end: 404,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31680,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 398,
                            end: 419,
                            as_str(): "assert(true != false)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 398,
                    end: 419,
                    as_str(): "assert(true != false)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e6588390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                    ),
                                    start: 430,
                                    end: 434,
                                    as_str(): "zero",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    B256(
                                        [
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                        ],
                                    ),
                                ),
                                return_type: TypeId(
                                    59,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e6588390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                    ),
                                    start: 437,
                                    end: 503,
                                    as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                59,
                            ),
                            type_ascription: TypeId(
                                31681,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 426,
                    end: 504,
                    as_str(): "let zero = 0x0000000000000000000000000000000000000000000000000000000000000000;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0e6588390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                    ),
                                    start: 513,
                                    end: 516,
                                    as_str(): "one",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    B256(
                                        [
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            1,
                                        ],
                                    ),
                                ),
                                return_type: TypeId(
                                    59,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e6588390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                    ),
                                    start: 519,
                                    end: 585,
                                    as_str(): "0x0000000000000000000000000000000000000000000000000000000000000001",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                59,
                            ),
                            type_ascription: TypeId(
                                31682,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 509,
                    end: 586,
                    as_str(): "let one = 0x0000000000000000000000000000000000000000000000000000000000000001;",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 591,
                                        end: 597,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 603,
                                                            end: 605,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 603,
                                                            end: 605,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 603,
                                                        end: 605,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3390,
                                                            end: 3394,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e6588390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                    ),
                                                                    start: 430,
                                                                    end: 434,
                                                                    as_str(): "zero",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e6588390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                ),
                                                                start: 598,
                                                                end: 602,
                                                                as_str(): "zero",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 598,
                                                            end: 602,
                                                            as_str(): "zero",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3396,
                                                            end: 3401,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e6588390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                    ),
                                                                    start: 430,
                                                                    end: 434,
                                                                    as_str(): "zero",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e6588390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                ),
                                                                start: 606,
                                                                end: 610,
                                                                as_str(): "zero",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 606,
                                                            end: 610,
                                                            as_str(): "zero",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13345,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3384,
                                                    end: 3636,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        // Both self and other are addresses of the values, so we can use MEQ.\n        asm(r1: self, r2: other, r3, r4) {\n            addi r3 zero i32;\n            meq r4 r1 r2 r3;\n            r4: bool\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 603,
                                                        end: 605,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 598,
                                            end: 610,
                                            as_str(): "zero == zero",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13346,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 591,
                                        end: 597,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31687,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 591,
                            end: 611,
                            as_str(): "assert(zero == zero)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 591,
                    end: 611,
                    as_str(): "assert(zero == zero)",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 617,
                                        end: 623,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 629,
                                                            end: 631,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 629,
                                                            end: 631,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 629,
                                                        end: 631,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e6588390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                    ),
                                                                    start: 430,
                                                                    end: 434,
                                                                    as_str(): "zero",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e6588390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                ),
                                                                start: 624,
                                                                end: 628,
                                                                as_str(): "zero",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 624,
                                                            end: 628,
                                                            as_str(): "zero",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e6588390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                    ),
                                                                    start: 513,
                                                                    end: 516,
                                                                    as_str(): "one",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e6588390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                ),
                                                                start: 632,
                                                                end: 635,
                                                                as_str(): "one",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 632,
                                                            end: 635,
                                                            as_str(): "one",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13348,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 629,
                                                        end: 631,
                                                        as_str(): "!=",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 624,
                                            end: 635,
                                            as_str(): "zero != one",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13349,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 617,
                                        end: 623,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31692,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 617,
                            end: 636,
                            as_str(): "assert(zero != one)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 617,
                    end: 636,
                    as_str(): "assert(zero != one)",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 663,
                                        end: 669,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 693,
                                                            end: 695,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 693,
                                                            end: 695,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 693,
                                                        end: 695,
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
                                                            src (ptr): 0x00007fb0ee184ed0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                                            ),
                                                            start: 305,
                                                            end: 309,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 670,
                                                                            end: 680,
                                                                            as_str(): "ContractId",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 682,
                                                                        end: 686,
                                                                        as_str(): "from",
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
                                                                            src (ptr): 0x00007fb0ee184ed0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                                                            ),
                                                                            start: 491,
                                                                            end: 495,
                                                                            as_str(): "bits",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 430,
                                                                                    end: 434,
                                                                                    as_str(): "zero",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 687,
                                                                                end: 691,
                                                                                as_str(): "zero",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            59,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 687,
                                                                            end: 691,
                                                                            as_str(): "zero",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13351,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee184ed0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                                                    ),
                                                                    start: 483,
                                                                    end: 559,
                                                                    as_str(): "fn from(bits: b256) -> ContractId {\n        ContractId { value: bits }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 670,
                                                                        end: 686,
                                                                        as_str(): "ContractId::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            7861,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 670,
                                                            end: 692,
                                                            as_str(): "ContractId::from(zero)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee184ed0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                                            ),
                                                            start: 311,
                                                            end: 316,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 696,
                                                                            end: 706,
                                                                            as_str(): "ContractId",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 708,
                                                                        end: 712,
                                                                        as_str(): "from",
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
                                                                            src (ptr): 0x00007fb0ee184ed0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                                                            ),
                                                                            start: 491,
                                                                            end: 495,
                                                                            as_str(): "bits",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 430,
                                                                                    end: 434,
                                                                                    as_str(): "zero",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 713,
                                                                                end: 717,
                                                                                as_str(): "zero",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            59,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 713,
                                                                            end: 717,
                                                                            as_str(): "zero",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13352,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee184ed0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                                                    ),
                                                                    start: 483,
                                                                    end: 559,
                                                                    as_str(): "fn from(bits: b256) -> ContractId {\n        ContractId { value: bits }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 696,
                                                                        end: 712,
                                                                        as_str(): "ContractId::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            7861,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 696,
                                                            end: 718,
                                                            as_str(): "ContractId::from(zero)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13353,
                                                Span {
                                                    src (ptr): 0x00007fb0ee184ed0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                                    ),
                                                    start: 299,
                                                    end: 373,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        self.value == other.value\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 693,
                                                        end: 695,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 670,
                                            end: 718,
                                            as_str(): "ContractId::from(zero) == ContractId::from(zero)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13354,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 663,
                                        end: 669,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31701,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 663,
                            end: 719,
                            as_str(): "assert(ContractId::from(zero) == ContractId::from(zero))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 663,
                    end: 719,
                    as_str(): "assert(ContractId::from(zero) == ContractId::from(zero))",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 725,
                                        end: 731,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 755,
                                                            end: 757,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 755,
                                                            end: 757,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 755,
                                                        end: 757,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 732,
                                                                            end: 742,
                                                                            as_str(): "ContractId",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 744,
                                                                        end: 748,
                                                                        as_str(): "from",
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
                                                                            src (ptr): 0x00007fb0ee184ed0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                                                            ),
                                                                            start: 491,
                                                                            end: 495,
                                                                            as_str(): "bits",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 430,
                                                                                    end: 434,
                                                                                    as_str(): "zero",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 749,
                                                                                end: 753,
                                                                                as_str(): "zero",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            59,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 749,
                                                                            end: 753,
                                                                            as_str(): "zero",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13356,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee184ed0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                                                    ),
                                                                    start: 483,
                                                                    end: 559,
                                                                    as_str(): "fn from(bits: b256) -> ContractId {\n        ContractId { value: bits }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 732,
                                                                        end: 748,
                                                                        as_str(): "ContractId::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            7861,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 732,
                                                            end: 754,
                                                            as_str(): "ContractId::from(zero)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 758,
                                                                            end: 768,
                                                                            as_str(): "ContractId",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 770,
                                                                        end: 774,
                                                                        as_str(): "from",
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
                                                                            src (ptr): 0x00007fb0ee184ed0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                                                            ),
                                                                            start: 491,
                                                                            end: 495,
                                                                            as_str(): "bits",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 513,
                                                                                    end: 516,
                                                                                    as_str(): "one",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 775,
                                                                                end: 778,
                                                                                as_str(): "one",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            59,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 775,
                                                                            end: 778,
                                                                            as_str(): "one",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13357,
                                                                Span {
                                                                    src (ptr): 0x00007fb0ee184ed0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                                                    ),
                                                                    start: 483,
                                                                    end: 559,
                                                                    as_str(): "fn from(bits: b256) -> ContractId {\n        ContractId { value: bits }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 758,
                                                                        end: 774,
                                                                        as_str(): "ContractId::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            7861,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 758,
                                                            end: 779,
                                                            as_str(): "ContractId::from(one)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13358,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 755,
                                                        end: 757,
                                                        as_str(): "!=",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 732,
                                            end: 779,
                                            as_str(): "ContractId::from(zero) != ContractId::from(one)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13359,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 725,
                                        end: 731,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31710,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 725,
                            end: 780,
                            as_str(): "assert(ContractId::from(zero) != ContractId::from(one))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 725,
                    end: 780,
                    as_str(): "assert(ContractId::from(zero) != ContractId::from(one))",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 787,
                                        end: 793,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 814,
                                                            end: 816,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 814,
                                                            end: 816,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 814,
                                                        end: 816,
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
                                                            src (ptr): 0x00007fb0f0722d40,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                            ),
                                                            start: 261,
                                                            end: 265,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 794,
                                                                            end: 801,
                                                                            as_str(): "Address",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 803,
                                                                        end: 807,
                                                                        as_str(): "from",
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
                                                                            src (ptr): 0x00007fb0f0722d40,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                                            ),
                                                                            start: 441,
                                                                            end: 445,
                                                                            as_str(): "bits",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 430,
                                                                                    end: 434,
                                                                                    as_str(): "zero",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 808,
                                                                                end: 812,
                                                                                as_str(): "zero",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            59,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 808,
                                                                            end: 812,
                                                                            as_str(): "zero",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13361,
                                                                Span {
                                                                    src (ptr): 0x00007fb0f0722d40,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                                    ),
                                                                    start: 433,
                                                                    end: 503,
                                                                    as_str(): "fn from(bits: b256) -> Address {\n        Address { value: bits }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 794,
                                                                        end: 807,
                                                                        as_str(): "Address::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            9112,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 794,
                                                            end: 813,
                                                            as_str(): "Address::from(zero)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f0722d40,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                            ),
                                                            start: 267,
                                                            end: 272,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 817,
                                                                            end: 824,
                                                                            as_str(): "Address",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 826,
                                                                        end: 830,
                                                                        as_str(): "from",
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
                                                                            src (ptr): 0x00007fb0f0722d40,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                                            ),
                                                                            start: 441,
                                                                            end: 445,
                                                                            as_str(): "bits",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 430,
                                                                                    end: 434,
                                                                                    as_str(): "zero",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 831,
                                                                                end: 835,
                                                                                as_str(): "zero",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            59,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 831,
                                                                            end: 835,
                                                                            as_str(): "zero",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13362,
                                                                Span {
                                                                    src (ptr): 0x00007fb0f0722d40,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                                    ),
                                                                    start: 433,
                                                                    end: 503,
                                                                    as_str(): "fn from(bits: b256) -> Address {\n        Address { value: bits }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 817,
                                                                        end: 830,
                                                                        as_str(): "Address::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            9112,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 817,
                                                            end: 836,
                                                            as_str(): "Address::from(zero)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13363,
                                                Span {
                                                    src (ptr): 0x00007fb0f0722d40,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                    ),
                                                    start: 255,
                                                    end: 329,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        self.value == other.value\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 814,
                                                        end: 816,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 794,
                                            end: 836,
                                            as_str(): "Address::from(zero) == Address::from(zero)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13364,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 787,
                                        end: 793,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31719,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 787,
                            end: 837,
                            as_str(): "assert(Address::from(zero) == Address::from(zero))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 787,
                    end: 837,
                    as_str(): "assert(Address::from(zero) == Address::from(zero))",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 843,
                                        end: 849,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 870,
                                                            end: 872,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 870,
                                                            end: 872,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 870,
                                                        end: 872,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 850,
                                                                            end: 857,
                                                                            as_str(): "Address",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 859,
                                                                        end: 863,
                                                                        as_str(): "from",
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
                                                                            src (ptr): 0x00007fb0f0722d40,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                                            ),
                                                                            start: 441,
                                                                            end: 445,
                                                                            as_str(): "bits",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 430,
                                                                                    end: 434,
                                                                                    as_str(): "zero",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 864,
                                                                                end: 868,
                                                                                as_str(): "zero",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            59,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 864,
                                                                            end: 868,
                                                                            as_str(): "zero",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13366,
                                                                Span {
                                                                    src (ptr): 0x00007fb0f0722d40,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                                    ),
                                                                    start: 433,
                                                                    end: 503,
                                                                    as_str(): "fn from(bits: b256) -> Address {\n        Address { value: bits }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 850,
                                                                        end: 863,
                                                                        as_str(): "Address::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            9112,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 850,
                                                            end: 869,
                                                            as_str(): "Address::from(zero)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 873,
                                                                            end: 880,
                                                                            as_str(): "Address",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 882,
                                                                        end: 886,
                                                                        as_str(): "from",
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
                                                                            src (ptr): 0x00007fb0f0722d40,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                                            ),
                                                                            start: 441,
                                                                            end: 445,
                                                                            as_str(): "bits",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e6588390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                    ),
                                                                                    start: 513,
                                                                                    end: 516,
                                                                                    as_str(): "one",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                ),
                                                                                start: 887,
                                                                                end: 890,
                                                                                as_str(): "one",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            59,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 887,
                                                                            end: 890,
                                                                            as_str(): "one",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13367,
                                                                Span {
                                                                    src (ptr): 0x00007fb0f0722d40,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                                    ),
                                                                    start: 433,
                                                                    end: 503,
                                                                    as_str(): "fn from(bits: b256) -> Address {\n        Address { value: bits }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 873,
                                                                        end: 886,
                                                                        as_str(): "Address::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            9112,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 873,
                                                            end: 891,
                                                            as_str(): "Address::from(one)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13368,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 870,
                                                        end: 872,
                                                        as_str(): "!=",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 850,
                                            end: 891,
                                            as_str(): "Address::from(zero) != Address::from(one)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13369,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 843,
                                        end: 849,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31728,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 843,
                            end: 892,
                            as_str(): "assert(Address::from(zero) != Address::from(one))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 843,
                    end: 892,
                    as_str(): "assert(Address::from(zero) != Address::from(one))",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 899,
                                        end: 905,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 931,
                                                            end: 933,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 931,
                                                            end: 933,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 931,
                                                        end: 933,
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
                                                            src (ptr): 0x00007fb10d5ffb50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                            ),
                                                            start: 420,
                                                            end: 424,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 906,
                                                                            end: 910,
                                                                            as_str(): "B512",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 912,
                                                                        end: 916,
                                                                        as_str(): "from",
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
                                                                            src (ptr): 0x00007fb10d5ffb50,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                            ),
                                                                            start: 656,
                                                                            end: 666,
                                                                            as_str(): "components",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Tuple {
                                                                            fields: [
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 430,
                                                                                                end: 434,
                                                                                                as_str(): "zero",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 918,
                                                                                            end: 922,
                                                                                            as_str(): "zero",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        59,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                        ),
                                                                                        start: 918,
                                                                                        end: 922,
                                                                                        as_str(): "zero",
                                                                                    },
                                                                                },
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 430,
                                                                                                end: 434,
                                                                                                as_str(): "zero",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 924,
                                                                                            end: 928,
                                                                                            as_str(): "zero",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        59,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                        ),
                                                                                        start: 924,
                                                                                        end: 928,
                                                                                        as_str(): "zero",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                        return_type: TypeId(
                                                                            31736,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 917,
                                                                            end: 929,
                                                                            as_str(): "(zero, zero)",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13371,
                                                                Span {
                                                                    src (ptr): 0x00007fb10d5ffb50,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                    ),
                                                                    start: 648,
                                                                    end: 771,
                                                                    as_str(): "fn from(components: (b256, b256)) -> B512 {\n        B512 {\n            bytes: [components.0, components.1],\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 906,
                                                                        end: 916,
                                                                        as_str(): "B512::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            8978,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 906,
                                                            end: 930,
                                                            as_str(): "B512::from((zero, zero))",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb10d5ffb50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                            ),
                                                            start: 426,
                                                            end: 431,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 934,
                                                                            end: 938,
                                                                            as_str(): "B512",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 940,
                                                                        end: 944,
                                                                        as_str(): "from",
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
                                                                            src (ptr): 0x00007fb10d5ffb50,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                            ),
                                                                            start: 656,
                                                                            end: 666,
                                                                            as_str(): "components",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Tuple {
                                                                            fields: [
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 430,
                                                                                                end: 434,
                                                                                                as_str(): "zero",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 946,
                                                                                            end: 950,
                                                                                            as_str(): "zero",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        59,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                        ),
                                                                                        start: 946,
                                                                                        end: 950,
                                                                                        as_str(): "zero",
                                                                                    },
                                                                                },
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 430,
                                                                                                end: 434,
                                                                                                as_str(): "zero",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 952,
                                                                                            end: 956,
                                                                                            as_str(): "zero",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        59,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                        ),
                                                                                        start: 952,
                                                                                        end: 956,
                                                                                        as_str(): "zero",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                        return_type: TypeId(
                                                                            31743,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 945,
                                                                            end: 957,
                                                                            as_str(): "(zero, zero)",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13372,
                                                                Span {
                                                                    src (ptr): 0x00007fb10d5ffb50,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                    ),
                                                                    start: 648,
                                                                    end: 771,
                                                                    as_str(): "fn from(components: (b256, b256)) -> B512 {\n        B512 {\n            bytes: [components.0, components.1],\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 934,
                                                                        end: 944,
                                                                        as_str(): "B512::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            8978,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 934,
                                                            end: 958,
                                                            as_str(): "B512::from((zero, zero))",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13373,
                                                Span {
                                                    src (ptr): 0x00007fb10d5ffb50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                    ),
                                                    start: 414,
                                                    end: 537,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        (self.bytes)[0] == (other.bytes)[0] && (self.bytes)[1] == (other.bytes)[1]\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 931,
                                                        end: 933,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 906,
                                            end: 958,
                                            as_str(): "B512::from((zero, zero)) == B512::from((zero, zero))",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13374,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 899,
                                        end: 905,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31745,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 899,
                            end: 959,
                            as_str(): "assert(B512::from((zero, zero)) == B512::from((zero, zero)))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 899,
                    end: 959,
                    as_str(): "assert(B512::from((zero, zero)) == B512::from((zero, zero)))",
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 965,
                                        end: 971,
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
                                            src (ptr): 0x00007fb0eed62b40,
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
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 997,
                                                            end: 999,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 997,
                                                            end: 999,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 997,
                                                        end: 999,
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
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 972,
                                                                            end: 976,
                                                                            as_str(): "B512",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 978,
                                                                        end: 982,
                                                                        as_str(): "from",
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
                                                                            src (ptr): 0x00007fb10d5ffb50,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                            ),
                                                                            start: 656,
                                                                            end: 666,
                                                                            as_str(): "components",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Tuple {
                                                                            fields: [
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 430,
                                                                                                end: 434,
                                                                                                as_str(): "zero",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 984,
                                                                                            end: 988,
                                                                                            as_str(): "zero",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        59,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                        ),
                                                                                        start: 984,
                                                                                        end: 988,
                                                                                        as_str(): "zero",
                                                                                    },
                                                                                },
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 430,
                                                                                                end: 434,
                                                                                                as_str(): "zero",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 990,
                                                                                            end: 994,
                                                                                            as_str(): "zero",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        59,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                        ),
                                                                                        start: 990,
                                                                                        end: 994,
                                                                                        as_str(): "zero",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                        return_type: TypeId(
                                                                            31753,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 983,
                                                                            end: 995,
                                                                            as_str(): "(zero, zero)",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13376,
                                                                Span {
                                                                    src (ptr): 0x00007fb10d5ffb50,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                    ),
                                                                    start: 648,
                                                                    end: 771,
                                                                    as_str(): "fn from(components: (b256, b256)) -> B512 {\n        B512 {\n            bytes: [components.0, components.1],\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 972,
                                                                        end: 982,
                                                                        as_str(): "B512::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            8978,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 972,
                                                            end: 996,
                                                            as_str(): "B512::from((zero, zero))",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f9ca5f30,
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
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 1000,
                                                                            end: 1004,
                                                                            as_str(): "B512",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 1006,
                                                                        end: 1010,
                                                                        as_str(): "from",
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
                                                                            src (ptr): 0x00007fb10d5ffb50,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                            ),
                                                                            start: 656,
                                                                            end: 666,
                                                                            as_str(): "components",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Tuple {
                                                                            fields: [
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 430,
                                                                                                end: 434,
                                                                                                as_str(): "zero",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 1012,
                                                                                            end: 1016,
                                                                                            as_str(): "zero",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        59,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                        ),
                                                                                        start: 1012,
                                                                                        end: 1016,
                                                                                        as_str(): "zero",
                                                                                    },
                                                                                },
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e6588390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                                ),
                                                                                                start: 513,
                                                                                                end: 516,
                                                                                                as_str(): "one",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e6588390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                            ),
                                                                                            start: 1018,
                                                                                            end: 1021,
                                                                                            as_str(): "one",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        59,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e6588390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                                        ),
                                                                                        start: 1018,
                                                                                        end: 1021,
                                                                                        as_str(): "one",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                        return_type: TypeId(
                                                                            31760,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e6588390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                            ),
                                                                            start: 1011,
                                                                            end: 1022,
                                                                            as_str(): "(zero, one)",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13377,
                                                                Span {
                                                                    src (ptr): 0x00007fb10d5ffb50,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                    ),
                                                                    start: 648,
                                                                    end: 771,
                                                                    as_str(): "fn from(components: (b256, b256)) -> B512 {\n        B512 {\n            bytes: [components.0, components.1],\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e6588390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                                        ),
                                                                        start: 1000,
                                                                        end: 1010,
                                                                        as_str(): "B512::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            8978,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e6588390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                            ),
                                                            start: 1000,
                                                            end: 1023,
                                                            as_str(): "B512::from((zero, one))",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13378,
                                                Span {
                                                    src (ptr): 0x00007fb0f9ca5f30,
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
                                                        src (ptr): 0x00007fb0e6588390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                                        ),
                                                        start: 997,
                                                        end: 999,
                                                        as_str(): "!=",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e6588390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                            ),
                                            start: 972,
                                            end: 1023,
                                            as_str(): "B512::from((zero, zero)) != B512::from((zero, one))",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13379,
                                Span {
                                    src (ptr): 0x00007fb0eed62b40,
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
                                        src (ptr): 0x00007fb0e6588390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                                        ),
                                        start: 965,
                                        end: 971,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31762,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 965,
                            end: 1024,
                            as_str(): "assert(B512::from((zero, zero)) != B512::from((zero, one)))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 965,
                    end: 1024,
                    as_str(): "assert(B512::from((zero, zero)) != B512::from((zero, one)))",
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
                            src (ptr): 0x00007fb0e6588390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                            ),
                            start: 1031,
                            end: 1035,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0e6588390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
                    ),
                    start: 1031,
                    end: 1035,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb0e6588390,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
        ),
        start: 117,
        end: 1037,
        as_str(): "fn main() -> bool {\n    // Primitive types\n    assert(1u8 == 1u8);\n    assert(1u8 != 2u8);\n\n    assert(1u16 == 1u16);\n    assert(1u16 != 2u16);\n\n    assert(1u32 == 1u32);\n    assert(1u32 != 2u32);\n\n    assert(1u64 == 1u64);\n    assert(1u64 != 2u64);\n\n    assert(true == true);\n    assert(true != false);\n\n    let zero = 0x0000000000000000000000000000000000000000000000000000000000000000;\n    let one = 0x0000000000000000000000000000000000000000000000000000000000000001;\n    assert(zero == zero);\n    assert(zero != one);\n\n    // stdlib types\n    assert(ContractId::from(zero) == ContractId::from(zero));\n    assert(ContractId::from(zero) != ContractId::from(one));\n\n    assert(Address::from(zero) == Address::from(zero));\n    assert(Address::from(zero) != Address::from(one));\n\n    assert(B512::from((zero, zero)) == B512::from((zero, zero)));\n    assert(B512::from((zero, zero)) != B512::from((zero, one)));\n\n    true\n}",
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
        src (ptr): 0x00007fb0e6588390,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRPNpScn/eq_and_neq/src/main.sw",
        ),
        start: 130,
        end: 134,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

