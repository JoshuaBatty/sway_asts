


TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb110d20660,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
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
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 104,
                                    end: 105,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1106046e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                            ),
                                            start: 29,
                                            end: 35,
                                            as_str(): "NUMBER",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1106046e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/test_lib.sw",
                                        ),
                                        start: 29,
                                        end: 35,
                                        as_str(): "NUMBER",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    31632,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 118,
                                    end: 124,
                                    as_str(): "NUMBER",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31634,
                            ),
                            type_ascription: TypeId(
                                31634,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb110d20660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                    ),
                    start: 100,
                    end: 125,
                    as_str(): "let x = test_lib::NUMBER;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 134,
                                    end: 138,
                                    as_str(): "zero",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb132f4b720,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/constants.sw",
                                            ),
                                            start: 117,
                                            end: 126,
                                            as_str(): "ZERO_B256",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb132f4b720,
                                        path: Some(
                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/constants.sw",
                                        ),
                                        start: 117,
                                        end: 126,
                                        as_str(): "ZERO_B256",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    59,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 157,
                                    end: 166,
                                    as_str(): "ZERO_B256",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                59,
                            ),
                            type_ascription: TypeId(
                                31635,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb110d20660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                    ),
                    start: 130,
                    end: 167,
                    as_str(): "let zero = std::constants::ZERO_B256;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 176,
                                    end: 189,
                                    as_str(): "base_asset_id",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb132f4b720,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/constants.sw",
                                            ),
                                            start: 62,
                                            end: 75,
                                            as_str(): "BASE_ASSET_ID",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb132f4b720,
                                        path: Some(
                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/constants.sw",
                                        ),
                                        start: 62,
                                        end: 75,
                                        as_str(): "BASE_ASSET_ID",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    7861,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 208,
                                    end: 221,
                                    as_str(): "BASE_ASSET_ID",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7861,
                            ),
                            type_ascription: TypeId(
                                31636,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb110d20660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                    ),
                    start: 172,
                    end: 222,
                    as_str(): "let base_asset_id = std::constants::BASE_ASSET_ID;",
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
                                        src (ptr): 0x00007fb110d20660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                        ),
                                        start: 227,
                                        end: 233,
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
                                            src (ptr): 0x00007fb11f1a9150,
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
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 239,
                                                            end: 241,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 239,
                                                            end: 241,
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
                                                        src (ptr): 0x00007fb110d20660,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                        ),
                                                        start: 239,
                                                        end: 241,
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
                                                            src (ptr): 0x00007fb12755ee50,
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
                                                                    src (ptr): 0x00007fb110d20660,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                    ),
                                                                    start: 134,
                                                                    end: 138,
                                                                    as_str(): "zero",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 234,
                                                                end: 238,
                                                                as_str(): "zero",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 234,
                                                            end: 238,
                                                            as_str(): "zero",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12755ee50,
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
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 242,
                                                            end: 308,
                                                            as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13316,
                                                Span {
                                                    src (ptr): 0x00007fb12755ee50,
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
                                                        src (ptr): 0x00007fb110d20660,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                        ),
                                                        start: 239,
                                                        end: 241,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb110d20660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                            ),
                                            start: 234,
                                            end: 308,
                                            as_str(): "zero == 0x0000000000000000000000000000000000000000000000000000000000000000",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13317,
                                Span {
                                    src (ptr): 0x00007fb11f1a9150,
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
                                        src (ptr): 0x00007fb110d20660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                        ),
                                        start: 227,
                                        end: 233,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31641,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb110d20660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                            ),
                            start: 227,
                            end: 309,
                            as_str(): "assert(zero == 0x0000000000000000000000000000000000000000000000000000000000000000)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb110d20660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                    ),
                    start: 227,
                    end: 309,
                    as_str(): "assert(zero == 0x0000000000000000000000000000000000000000000000000000000000000000)",
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
                                        src (ptr): 0x00007fb110d20660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                        ),
                                        start: 315,
                                        end: 321,
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
                                            src (ptr): 0x00007fb11f1a9150,
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
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 336,
                                                            end: 338,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 336,
                                                            end: 338,
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
                                                        src (ptr): 0x00007fb110d20660,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                        ),
                                                        start: 336,
                                                        end: 338,
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
                                                            src (ptr): 0x00007fb12b383cf0,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb110d20660,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                    ),
                                                                    start: 176,
                                                                    end: 189,
                                                                    as_str(): "base_asset_id",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 322,
                                                                end: 335,
                                                                as_str(): "base_asset_id",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            7861,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 322,
                                                            end: 335,
                                                            as_str(): "base_asset_id",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b383cf0,
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
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 339,
                                                                            end: 349,
                                                                            as_str(): "ContractId",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 351,
                                                                        end: 355,
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
                                                                            src (ptr): 0x00007fb12b383cf0,
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
                                                                                    src (ptr): 0x00007fb110d20660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                    ),
                                                                                    start: 134,
                                                                                    end: 138,
                                                                                    as_str(): "zero",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb110d20660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                ),
                                                                                start: 356,
                                                                                end: 360,
                                                                                as_str(): "zero",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            59,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 356,
                                                                            end: 360,
                                                                            as_str(): "zero",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13319,
                                                                Span {
                                                                    src (ptr): 0x00007fb12b383cf0,
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
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 339,
                                                                        end: 355,
                                                                        as_str(): "ContractId::from",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            7861,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 339,
                                                            end: 361,
                                                            as_str(): "ContractId::from(zero)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13320,
                                                Span {
                                                    src (ptr): 0x00007fb12b383cf0,
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
                                                        src (ptr): 0x00007fb110d20660,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                        ),
                                                        start: 336,
                                                        end: 338,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb110d20660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                            ),
                                            start: 322,
                                            end: 361,
                                            as_str(): "base_asset_id == ContractId::from(zero)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13321,
                                Span {
                                    src (ptr): 0x00007fb11f1a9150,
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
                                        src (ptr): 0x00007fb110d20660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                        ),
                                        start: 315,
                                        end: 321,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31648,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb110d20660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                            ),
                            start: 315,
                            end: 362,
                            as_str(): "assert(base_asset_id == ContractId::from(zero))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb110d20660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                    ),
                    start: 315,
                    end: 362,
                    as_str(): "assert(base_asset_id == ContractId::from(zero))",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 104,
                                    end: 105,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fb110d20660,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                ),
                                start: 368,
                                end: 369,
                                as_str(): "x",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            31634,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb110d20660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                            ),
                            start: 368,
                            end: 369,
                            as_str(): "x",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb110d20660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                    ),
                    start: 368,
                    end: 369,
                    as_str(): "x",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb110d20660,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
        ),
        start: 77,
        end: 371,
        as_str(): "fn main() -> u64 {\n    let x = test_lib::NUMBER;\n    let zero = std::constants::ZERO_B256;\n    let base_asset_id = std::constants::BASE_ASSET_ID;\n    assert(zero == 0x0000000000000000000000000000000000000000000000000000000000000000);\n    assert(base_asset_id == ContractId::from(zero));\n    x\n}",
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
        src (ptr): 0x00007fb110d20660,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
        ),
        start: 90,
        end: 93,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

