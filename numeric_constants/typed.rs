
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe06890a930,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 59,
                                        end: 65,
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
                                            src (ptr): 0x00007fe0714d47c0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 77,
                                                            end: 79,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 77,
                                                            end: 79,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 77,
                                                        end: 79,
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
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 71,
                                                                        end: 74,
                                                                        as_str(): "max",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                13315,
                                                                Span {
                                                                    src (ptr): 0x00007fe0785222f0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/primitives.sw",
                                                                    ),
                                                                    start: 252,
                                                                    end: 308,
                                                                    as_str(): "pub fn max() -> u64 {\n        18446744073709551615\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 66,
                                                                        end: 74,
                                                                        as_str(): "u64::max",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 66,
                                                            end: 76,
                                                            as_str(): "u64::max()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                18446744073709551615,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 80,
                                                            end: 100,
                                                            as_str(): "18446744073709551615",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13316,
                                                Span {
                                                    src (ptr): 0x00007fe07079c0d0,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 77,
                                                        end: 79,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 100,
                                            as_str(): "u64::max() == 18446744073709551615",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13317,
                                Span {
                                    src (ptr): 0x00007fe0714d47c0,
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 59,
                                        end: 65,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31636,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 59,
                            end: 101,
                            as_str(): "assert(u64::max() == 18446744073709551615)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 59,
                    end: 101,
                    as_str(): "assert(u64::max() == 18446744073709551615)",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 107,
                                        end: 113,
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
                                            src (ptr): 0x00007fe0714d47c0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 125,
                                                            end: 127,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 125,
                                                            end: 127,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 125,
                                                        end: 127,
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
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 119,
                                                                        end: 122,
                                                                        as_str(): "min",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                13319,
                                                                Span {
                                                                    src (ptr): 0x00007fe0785222f0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/primitives.sw",
                                                                    ),
                                                                    start: 109,
                                                                    end: 146,
                                                                    as_str(): "pub fn min() -> u64 {\n        0\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 114,
                                                                        end: 122,
                                                                        as_str(): "u64::min",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 124,
                                                            as_str(): "u64::min()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 128,
                                                            end: 132,
                                                            as_str(): "0u64",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13320,
                                                Span {
                                                    src (ptr): 0x00007fe07079c0d0,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 125,
                                                        end: 127,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 114,
                                            end: 132,
                                            as_str(): "u64::min() == 0u64",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13321,
                                Span {
                                    src (ptr): 0x00007fe0714d47c0,
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 107,
                                        end: 113,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31641,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 107,
                            end: 133,
                            as_str(): "assert(u64::min() == 0u64)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 107,
                    end: 133,
                    as_str(): "assert(u64::min() == 0u64)",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 139,
                                        end: 145,
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
                                            src (ptr): 0x00007fe0714d47c0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 158,
                                                            end: 160,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 158,
                                                            end: 160,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 158,
                                                        end: 160,
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
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 151,
                                                                        end: 155,
                                                                        as_str(): "bits",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                13323,
                                                                Span {
                                                                    src (ptr): 0x00007fe0785222f0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/primitives.sw",
                                                                    ),
                                                                    start: 361,
                                                                    end: 400,
                                                                    as_str(): "pub fn bits() -> u32 {\n        64\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 146,
                                                                        end: 155,
                                                                        as_str(): "u64::bits",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 146,
                                                            end: 157,
                                                            as_str(): "u64::bits()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                64,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 161,
                                                            end: 166,
                                                            as_str(): "64u32",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13324,
                                                Span {
                                                    src (ptr): 0x00007fe07079c0d0,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 158,
                                                        end: 160,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 146,
                                            end: 166,
                                            as_str(): "u64::bits() == 64u32",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13325,
                                Span {
                                    src (ptr): 0x00007fe0714d47c0,
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 139,
                                        end: 145,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31646,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 139,
                            end: 167,
                            as_str(): "assert(u64::bits() == 64u32)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 139,
                    end: 167,
                    as_str(): "assert(u64::bits() == 64u32)",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 173,
                                        end: 179,
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
                                            src (ptr): 0x00007fe0714d47c0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 191,
                                                            end: 193,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 191,
                                                            end: 193,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 191,
                                                        end: 193,
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
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 185,
                                                                        end: 188,
                                                                        as_str(): "max",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                13327,
                                                                Span {
                                                                    src (ptr): 0x00007fe0785222f0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/primitives.sw",
                                                                    ),
                                                                    start: 635,
                                                                    end: 681,
                                                                    as_str(): "pub fn max() -> u32 {\n        4294967295\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 180,
                                                                        end: 188,
                                                                        as_str(): "u32::max",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 180,
                                                            end: 190,
                                                            as_str(): "u32::max()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                4294967295,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 207,
                                                            as_str(): "4294967295u32",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13328,
                                                Span {
                                                    src (ptr): 0x00007fe07079c0d0,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 191,
                                                        end: 193,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 180,
                                            end: 207,
                                            as_str(): "u32::max() == 4294967295u32",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13329,
                                Span {
                                    src (ptr): 0x00007fe0714d47c0,
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 173,
                                        end: 179,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31651,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 173,
                            end: 208,
                            as_str(): "assert(u32::max() == 4294967295u32)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 173,
                    end: 208,
                    as_str(): "assert(u32::max() == 4294967295u32)",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 214,
                                        end: 220,
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
                                            src (ptr): 0x00007fe0714d47c0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 232,
                                                            end: 234,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 232,
                                                            end: 234,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 232,
                                                        end: 234,
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
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 226,
                                                                        end: 229,
                                                                        as_str(): "min",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                13331,
                                                                Span {
                                                                    src (ptr): 0x00007fe0785222f0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/primitives.sw",
                                                                    ),
                                                                    start: 492,
                                                                    end: 529,
                                                                    as_str(): "pub fn min() -> u32 {\n        0\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 221,
                                                                        end: 229,
                                                                        as_str(): "u32::min",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 221,
                                                            end: 231,
                                                            as_str(): "u32::min()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                0,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 235,
                                                            end: 239,
                                                            as_str(): "0u32",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13332,
                                                Span {
                                                    src (ptr): 0x00007fe07079c0d0,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 232,
                                                        end: 234,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 221,
                                            end: 239,
                                            as_str(): "u32::min() == 0u32",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13333,
                                Span {
                                    src (ptr): 0x00007fe0714d47c0,
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 214,
                                        end: 220,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31656,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 214,
                            end: 240,
                            as_str(): "assert(u32::min() == 0u32)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 214,
                    end: 240,
                    as_str(): "assert(u32::min() == 0u32)",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 246,
                                        end: 252,
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
                                            src (ptr): 0x00007fe0714d47c0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 267,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 267,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 265,
                                                        end: 267,
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
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 258,
                                                                        end: 262,
                                                                        as_str(): "bits",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                13335,
                                                                Span {
                                                                    src (ptr): 0x00007fe0785222f0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/primitives.sw",
                                                                    ),
                                                                    start: 734,
                                                                    end: 773,
                                                                    as_str(): "pub fn bits() -> u32 {\n        32\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 253,
                                                                        end: 262,
                                                                        as_str(): "u32::bits",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 253,
                                                            end: 264,
                                                            as_str(): "u32::bits()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                            U16(
                                                                32,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 268,
                                                            end: 273,
                                                            as_str(): "32u16",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13336,
                                                Span {
                                                    src (ptr): 0x00007fe07079c0d0,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 265,
                                                        end: 267,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 253,
                                            end: 273,
                                            as_str(): "u32::bits() == 32u16",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13337,
                                Span {
                                    src (ptr): 0x00007fe0714d47c0,
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 246,
                                        end: 252,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31661,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 246,
                            end: 274,
                            as_str(): "assert(u32::bits() == 32u16)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 246,
                    end: 274,
                    as_str(): "assert(u32::bits() == 32u16)",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 280,
                                        end: 286,
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
                                            src (ptr): 0x00007fe0714d47c0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 298,
                                                            end: 300,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 298,
                                                            end: 300,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 298,
                                                        end: 300,
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
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 292,
                                                                        end: 295,
                                                                        as_str(): "max",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                13339,
                                                                Span {
                                                                    src (ptr): 0x00007fe0785222f0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/primitives.sw",
                                                                    ),
                                                                    start: 1008,
                                                                    end: 1049,
                                                                    as_str(): "pub fn max() -> u16 {\n        65535\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 287,
                                                                        end: 295,
                                                                        as_str(): "u16::max",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 287,
                                                            end: 297,
                                                            as_str(): "u16::max()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                65535,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 301,
                                                            end: 309,
                                                            as_str(): "65535u16",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13340,
                                                Span {
                                                    src (ptr): 0x00007fe07079c0d0,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 298,
                                                        end: 300,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 287,
                                            end: 309,
                                            as_str(): "u16::max() == 65535u16",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13341,
                                Span {
                                    src (ptr): 0x00007fe0714d47c0,
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 280,
                                        end: 286,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31666,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 280,
                            end: 310,
                            as_str(): "assert(u16::max() == 65535u16)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 280,
                    end: 310,
                    as_str(): "assert(u16::max() == 65535u16)",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 316,
                                        end: 322,
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
                                            src (ptr): 0x00007fe0714d47c0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 334,
                                                            end: 336,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 334,
                                                            end: 336,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 334,
                                                        end: 336,
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
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 328,
                                                                        end: 331,
                                                                        as_str(): "min",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                13343,
                                                                Span {
                                                                    src (ptr): 0x00007fe0785222f0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/primitives.sw",
                                                                    ),
                                                                    start: 865,
                                                                    end: 902,
                                                                    as_str(): "pub fn min() -> u16 {\n        0\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 323,
                                                                        end: 331,
                                                                        as_str(): "u16::min",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 323,
                                                            end: 333,
                                                            as_str(): "u16::min()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                0,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 337,
                                                            end: 341,
                                                            as_str(): "0u16",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13344,
                                                Span {
                                                    src (ptr): 0x00007fe07079c0d0,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 334,
                                                        end: 336,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 323,
                                            end: 341,
                                            as_str(): "u16::min() == 0u16",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13345,
                                Span {
                                    src (ptr): 0x00007fe0714d47c0,
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 316,
                                        end: 322,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31671,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 316,
                            end: 342,
                            as_str(): "assert(u16::min() == 0u16)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 316,
                    end: 342,
                    as_str(): "assert(u16::min() == 0u16)",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 348,
                                        end: 354,
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
                                            src (ptr): 0x00007fe0714d47c0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 367,
                                                            end: 369,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 367,
                                                            end: 369,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 367,
                                                        end: 369,
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
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 360,
                                                                        end: 364,
                                                                        as_str(): "bits",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                13347,
                                                                Span {
                                                                    src (ptr): 0x00007fe0785222f0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/primitives.sw",
                                                                    ),
                                                                    start: 1102,
                                                                    end: 1141,
                                                                    as_str(): "pub fn bits() -> u32 {\n        16\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 355,
                                                                        end: 364,
                                                                        as_str(): "u16::bits",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 355,
                                                            end: 366,
                                                            as_str(): "u16::bits()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                            U8(
                                                                16,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 370,
                                                            end: 374,
                                                            as_str(): "16u8",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13348,
                                                Span {
                                                    src (ptr): 0x00007fe07079c0d0,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 367,
                                                        end: 369,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 355,
                                            end: 374,
                                            as_str(): "u16::bits() == 16u8",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13349,
                                Span {
                                    src (ptr): 0x00007fe0714d47c0,
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 348,
                                        end: 354,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31676,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 348,
                            end: 375,
                            as_str(): "assert(u16::bits() == 16u8)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 348,
                    end: 375,
                    as_str(): "assert(u16::bits() == 16u8)",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 381,
                                        end: 387,
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
                                            src (ptr): 0x00007fe0714d47c0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 398,
                                                            end: 400,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 398,
                                                            end: 400,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 398,
                                                        end: 400,
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
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 392,
                                                                        end: 395,
                                                                        as_str(): "max",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                13351,
                                                                Span {
                                                                    src (ptr): 0x00007fe0785222f0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/primitives.sw",
                                                                    ),
                                                                    start: 1373,
                                                                    end: 1411,
                                                                    as_str(): "pub fn max() -> u8 {\n        255\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 388,
                                                                        end: 395,
                                                                        as_str(): "u8::max",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 388,
                                                            end: 397,
                                                            as_str(): "u8::max()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                255,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 401,
                                                            end: 406,
                                                            as_str(): "255u8",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13352,
                                                Span {
                                                    src (ptr): 0x00007fe07079c0d0,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 398,
                                                        end: 400,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 388,
                                            end: 406,
                                            as_str(): "u8::max() == 255u8",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13353,
                                Span {
                                    src (ptr): 0x00007fe0714d47c0,
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 381,
                                        end: 387,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31681,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 381,
                            end: 407,
                            as_str(): "assert(u8::max() == 255u8)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 381,
                    end: 407,
                    as_str(): "assert(u8::max() == 255u8)",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 413,
                                        end: 419,
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
                                            src (ptr): 0x00007fe0714d47c0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 430,
                                                            end: 432,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 430,
                                                            end: 432,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 430,
                                                        end: 432,
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
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 424,
                                                                        end: 427,
                                                                        as_str(): "min",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                13355,
                                                                Span {
                                                                    src (ptr): 0x00007fe0785222f0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/primitives.sw",
                                                                    ),
                                                                    start: 1232,
                                                                    end: 1268,
                                                                    as_str(): "pub fn min() -> u8 {\n        0\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 420,
                                                                        end: 427,
                                                                        as_str(): "u8::min",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 420,
                                                            end: 429,
                                                            as_str(): "u8::min()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                0,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 433,
                                                            end: 436,
                                                            as_str(): "0u8",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13356,
                                                Span {
                                                    src (ptr): 0x00007fe07079c0d0,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 430,
                                                        end: 432,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 420,
                                            end: 436,
                                            as_str(): "u8::min() == 0u8",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13357,
                                Span {
                                    src (ptr): 0x00007fe0714d47c0,
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 413,
                                        end: 419,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31686,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 413,
                            end: 437,
                            as_str(): "assert(u8::min() == 0u8)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 413,
                    end: 437,
                    as_str(): "assert(u8::min() == 0u8)",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 443,
                                        end: 449,
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
                                            src (ptr): 0x00007fe0714d47c0,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 461,
                                                            end: 463,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 461,
                                                            end: 463,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 461,
                                                        end: 463,
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
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 454,
                                                                        end: 458,
                                                                        as_str(): "bits",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                13359,
                                                                Span {
                                                                    src (ptr): 0x00007fe0785222f0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/primitives.sw",
                                                                    ),
                                                                    start: 1464,
                                                                    end: 1502,
                                                                    as_str(): "pub fn bits() -> u32 {\n        8\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 450,
                                                                        end: 458,
                                                                        as_str(): "u8::bits",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 450,
                                                            end: 460,
                                                            as_str(): "u8::bits()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07079c0d0,
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
                                                                8,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 464,
                                                            end: 468,
                                                            as_str(): "8u32",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13360,
                                                Span {
                                                    src (ptr): 0x00007fe07079c0d0,
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
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 461,
                                                        end: 463,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 450,
                                            end: 468,
                                            as_str(): "u8::bits() == 8u32",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13361,
                                Span {
                                    src (ptr): 0x00007fe0714d47c0,
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 443,
                                        end: 449,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31691,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 443,
                            end: 469,
                            as_str(): "assert(u8::bits() == 8u32)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 443,
                    end: 469,
                    as_str(): "assert(u8::bits() == 8u32)",
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
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 476,
                            end: 480,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06890a930,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                    ),
                    start: 476,
                    end: 480,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe06890a930,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
        ),
        start: 35,
        end: 482,
        as_str(): "fn main() -> bool {\n    assert(u64::max() == 18446744073709551615);\n    assert(u64::min() == 0u64);\n    assert(u64::bits() == 64u32);\n    assert(u32::max() == 4294967295u32);\n    assert(u32::min() == 0u32);\n    assert(u32::bits() == 32u16);\n    assert(u16::max() == 65535u16);\n    assert(u16::min() == 0u16);\n    assert(u16::bits() == 16u8);\n    assert(u8::max() == 255u8);\n    assert(u8::min() == 0u8);\n    assert(u8::bits() == 8u32);\n\n    true\n}",
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
        src (ptr): 0x00007fe06890a930,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
        ),
        start: 48,
        end: 52,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

