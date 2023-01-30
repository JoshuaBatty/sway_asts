TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb12632e1f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
            ),
            start: 12,
            end: 16,
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
                                        src (ptr): 0x00007fb12632e1f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                        ),
                                        start: 33,
                                        end: 39,
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
                                            src (ptr): 0x00007fb132388e40,
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
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 45,
                                                            end: 47,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 45,
                                                            end: 47,
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
                                                        src (ptr): 0x00007fb12632e1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                        ),
                                                        start: 45,
                                                        end: 47,
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
                                                            src (ptr): 0x00007fb13a9c1f00,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 40,
                                                                            end: 41,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 40,
                                                                            end: 41,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "not",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 40,
                                                                        end: 41,
                                                                        as_str(): "!",
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
                                                                            src (ptr): 0x00007fb13a9c1f00,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 9119,
                                                                            end: 9123,
                                                                            as_str(): "self",
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
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 41,
                                                                            end: 44,
                                                                            as_str(): "2u8",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13315,
                                                                Span {
                                                                    src (ptr): 0x00007fb13a9c1f00,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 9112,
                                                                    end: 9265,
                                                                    as_str(): "fn not(self) -> Self {\n        asm(r1: self, r2, r3: u8::max(), r4) {\n            not r2 r1;\n            and r4 r2 r3;\n            r4: u8\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 40,
                                                                        end: 41,
                                                                        as_str(): "!",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 40,
                                                            end: 44,
                                                            as_str(): "!2u8",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb13a9c1f00,
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
                                                                253,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 48,
                                                            end: 53,
                                                            as_str(): "253u8",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13316,
                                                Span {
                                                    src (ptr): 0x00007fb13a9c1f00,
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
                                                        src (ptr): 0x00007fb12632e1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                        ),
                                                        start: 45,
                                                        end: 47,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12632e1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 53,
                                            as_str(): "!2u8 == 253u8",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13317,
                                Span {
                                    src (ptr): 0x00007fb132388e40,
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
                                        src (ptr): 0x00007fb12632e1f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                        ),
                                        start: 33,
                                        end: 39,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31636,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12632e1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                            ),
                            start: 33,
                            end: 54,
                            as_str(): "assert(!2u8 == 253u8)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12632e1f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                    ),
                    start: 33,
                    end: 54,
                    as_str(): "assert(!2u8 == 253u8)",
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
                                        src (ptr): 0x00007fb12632e1f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                        ),
                                        start: 60,
                                        end: 66,
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
                                            src (ptr): 0x00007fb132388e40,
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
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 73,
                                                            end: 75,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 73,
                                                            end: 75,
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
                                                        src (ptr): 0x00007fb12632e1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                        ),
                                                        start: 73,
                                                        end: 75,
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
                                                            src (ptr): 0x00007fb13a9c1f00,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 67,
                                                                            end: 68,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 67,
                                                                            end: 68,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "not",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 67,
                                                                        end: 68,
                                                                        as_str(): "!",
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
                                                                            src (ptr): 0x00007fb13a9c1f00,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 8938,
                                                                            end: 8942,
                                                                            as_str(): "self",
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
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 68,
                                                                            end: 72,
                                                                            as_str(): "2u16",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13319,
                                                                Span {
                                                                    src (ptr): 0x00007fb13a9c1f00,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 8931,
                                                                    end: 9086,
                                                                    as_str(): "fn not(self) -> Self {\n        asm(r1: self, r2, r3: u16::max(), r4) {\n            not r2 r1;\n            and r4 r2 r3;\n            r4: u16\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 67,
                                                                        end: 68,
                                                                        as_str(): "!",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 67,
                                                            end: 72,
                                                            as_str(): "!2u16",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb13a9c1f00,
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
                                                                65533,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 76,
                                                            end: 84,
                                                            as_str(): "65533u16",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13320,
                                                Span {
                                                    src (ptr): 0x00007fb13a9c1f00,
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
                                                        src (ptr): 0x00007fb12632e1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                        ),
                                                        start: 73,
                                                        end: 75,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12632e1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                            ),
                                            start: 67,
                                            end: 84,
                                            as_str(): "!2u16 == 65533u16",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13321,
                                Span {
                                    src (ptr): 0x00007fb132388e40,
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
                                        src (ptr): 0x00007fb12632e1f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                        ),
                                        start: 60,
                                        end: 66,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31642,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12632e1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                            ),
                            start: 60,
                            end: 85,
                            as_str(): "assert(!2u16 == 65533u16)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12632e1f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                    ),
                    start: 60,
                    end: 85,
                    as_str(): "assert(!2u16 == 65533u16)",
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
                                        src (ptr): 0x00007fb12632e1f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                        ),
                                        start: 91,
                                        end: 97,
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
                                            src (ptr): 0x00007fb132388e40,
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
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                                        src (ptr): 0x00007fb12632e1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                                            src (ptr): 0x00007fb13a9c1f00,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 98,
                                                                            end: 99,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 98,
                                                                            end: 99,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "not",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 98,
                                                                        end: 99,
                                                                        as_str(): "!",
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
                                                                            src (ptr): 0x00007fb13a9c1f00,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 8756,
                                                                            end: 8760,
                                                                            as_str(): "self",
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
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 99,
                                                                            end: 103,
                                                                            as_str(): "2u32",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13323,
                                                                Span {
                                                                    src (ptr): 0x00007fb13a9c1f00,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 8749,
                                                                    end: 8904,
                                                                    as_str(): "fn not(self) -> Self {\n        asm(r1: self, r2, r3: u32::max(), r4) {\n            not r2 r1;\n            and r4 r2 r3;\n            r4: u32\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 98,
                                                                        end: 99,
                                                                        as_str(): "!",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 103,
                                                            as_str(): "!2u32",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb13a9c1f00,
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
                                                                4294967293,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 107,
                                                            end: 120,
                                                            as_str(): "4294967293u32",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13324,
                                                Span {
                                                    src (ptr): 0x00007fb13a9c1f00,
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
                                                        src (ptr): 0x00007fb12632e1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                            src (ptr): 0x00007fb12632e1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                            ),
                                            start: 98,
                                            end: 120,
                                            as_str(): "!2u32 == 4294967293u32",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13325,
                                Span {
                                    src (ptr): 0x00007fb132388e40,
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
                                        src (ptr): 0x00007fb12632e1f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                        ),
                                        start: 91,
                                        end: 97,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31648,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12632e1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                            ),
                            start: 91,
                            end: 121,
                            as_str(): "assert(!2u32 == 4294967293u32)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12632e1f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                    ),
                    start: 91,
                    end: 121,
                    as_str(): "assert(!2u32 == 4294967293u32)",
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
                                        src (ptr): 0x00007fb12632e1f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                        ),
                                        start: 127,
                                        end: 133,
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
                                            src (ptr): 0x00007fb132388e40,
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
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 140,
                                                            end: 142,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 140,
                                                            end: 142,
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
                                                        src (ptr): 0x00007fb12632e1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                        ),
                                                        start: 140,
                                                        end: 142,
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
                                                            src (ptr): 0x00007fb13a9c1f00,
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
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 134,
                                                                            end: 135,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 134,
                                                                            end: 135,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "not",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 134,
                                                                        end: 135,
                                                                        as_str(): "!",
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
                                                                            src (ptr): 0x00007fb13a9c1f00,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 8620,
                                                                            end: 8624,
                                                                            as_str(): "self",
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
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 135,
                                                                            end: 139,
                                                                            as_str(): "2u64",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13327,
                                                                Span {
                                                                    src (ptr): 0x00007fb13a9c1f00,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 8613,
                                                                    end: 8722,
                                                                    as_str(): "fn not(self) -> Self {\n        asm(r1: self, r2) {\n            not r2 r1;\n            r2: u64\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 134,
                                                                        end: 135,
                                                                        as_str(): "!",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 134,
                                                            end: 139,
                                                            as_str(): "!2u64",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb13a9c1f00,
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
                                                                18446744073709551613,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 143,
                                                            end: 166,
                                                            as_str(): "18446744073709551613u64",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13328,
                                                Span {
                                                    src (ptr): 0x00007fb13a9c1f00,
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
                                                        src (ptr): 0x00007fb12632e1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                        ),
                                                        start: 140,
                                                        end: 142,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12632e1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                            ),
                                            start: 134,
                                            end: 166,
                                            as_str(): "!2u64 == 18446744073709551613u64",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13329,
                                Span {
                                    src (ptr): 0x00007fb132388e40,
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
                                        src (ptr): 0x00007fb12632e1f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                        ),
                                        start: 127,
                                        end: 133,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31654,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12632e1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                            ),
                            start: 127,
                            end: 167,
                            as_str(): "assert(!2u64 == 18446744073709551613u64)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12632e1f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                    ),
                    start: 127,
                    end: 167,
                    as_str(): "assert(!2u64 == 18446744073709551613u64)",
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
                            src (ptr): 0x00007fb12632e1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                            ),
                            start: 174,
                            end: 178,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12632e1f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                    ),
                    start: 174,
                    end: 178,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb12632e1f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
        ),
        start: 9,
        end: 180,
        as_str(): "fn main() -> bool {\n    assert(!2u8 == 253u8);\n    assert(!2u16 == 65533u16);\n    assert(!2u32 == 4294967293u32);\n    assert(!2u64 == 18446744073709551613u64);\n\n    true\n}",
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
        src (ptr): 0x00007fb12632e1f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
        ),
        start: 22,
        end: 26,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

