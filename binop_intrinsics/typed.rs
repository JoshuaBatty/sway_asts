
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb14d3784f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 61,
                                    end: 62,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U8(
                                        2,
                                    ),
                                ),
                                return_type: TypeId(
                                    31631,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 69,
                                    end: 70,
                                    as_str(): "2",
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
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 64,
                                    end: 66,
                                    as_str(): "u8",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 57,
                    end: 71,
                    as_str(): "let a: u8 = 2;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 78,
                                    end: 79,
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
                                    31632,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 86,
                                    end: 88,
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
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 81,
                                    end: 83,
                                    as_str(): "u8",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 74,
                    end: 89,
                    as_str(): "let b: u8 = 22;",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 92,
                                        end: 98,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 111,
                                                            end: 113,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 111,
                                                            end: 113,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 111,
                                                        end: 113,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Add,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 61,
                                                                                    end: 62,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 105,
                                                                                end: 106,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 105,
                                                                            end: 106,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 78,
                                                                                    end: 79,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 108,
                                                                                end: 109,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 108,
                                                                            end: 109,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 99,
                                                                    end: 110,
                                                                    as_str(): "__add(a, b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 99,
                                                            end: 110,
                                                            as_str(): "__add(a, b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            U64(
                                                                24,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 116,
                                                            as_str(): "24",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13315,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 111,
                                                        end: 113,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 99,
                                            end: 116,
                                            as_str(): "__add(a, b) == 24",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13316,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 92,
                                        end: 98,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31639,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 92,
                            end: 117,
                            as_str(): "assert(__add(a, b) == 24)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 92,
                    end: 117,
                    as_str(): "assert(__add(a, b) == 24)",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 121,
                                        end: 127,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Sub,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 78,
                                                                                    end: 79,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 134,
                                                                                end: 135,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 134,
                                                                            end: 135,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 61,
                                                                                    end: 62,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 137,
                                                                                end: 138,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 137,
                                                                            end: 138,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 128,
                                                                    end: 139,
                                                                    as_str(): "__sub(b, a)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 128,
                                                            end: 139,
                                                            as_str(): "__sub(b, a)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            U64(
                                                                20,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 143,
                                                            end: 145,
                                                            as_str(): "20",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13318,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 145,
                                            as_str(): "__sub(b, a) == 20",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13319,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 121,
                                        end: 127,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31646,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 121,
                            end: 146,
                            as_str(): "assert(__sub(b, a) == 20)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 121,
                    end: 146,
                    as_str(): "assert(__sub(b, a) == 20)",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 150,
                                        end: 156,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 171,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 171,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 169,
                                                        end: 171,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Mul,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 61,
                                                                                    end: 62,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 163,
                                                                                end: 164,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 163,
                                                                            end: 164,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 78,
                                                                                    end: 79,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 166,
                                                                                end: 167,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 166,
                                                                            end: 167,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 157,
                                                                    end: 168,
                                                                    as_str(): "__mul(a, b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 157,
                                                            end: 168,
                                                            as_str(): "__mul(a, b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            U64(
                                                                44,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 174,
                                                            as_str(): "44",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13321,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 169,
                                                        end: 171,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 157,
                                            end: 174,
                                            as_str(): "__mul(a, b) == 44",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13322,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 150,
                                        end: 156,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31653,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 150,
                            end: 175,
                            as_str(): "assert(__mul(a, b) == 44)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 150,
                    end: 175,
                    as_str(): "assert(__mul(a, b) == 44)",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 179,
                                        end: 185,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 198,
                                                            end: 200,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 198,
                                                            end: 200,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 198,
                                                        end: 200,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Div,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 78,
                                                                                    end: 79,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 192,
                                                                                end: 193,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 192,
                                                                            end: 193,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 61,
                                                                                    end: 62,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 195,
                                                                                end: 196,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 195,
                                                                            end: 196,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 186,
                                                                    end: 197,
                                                                    as_str(): "__div(b, a)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 186,
                                                            end: 197,
                                                            as_str(): "__div(b, a)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            U64(
                                                                11,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 201,
                                                            end: 203,
                                                            as_str(): "11",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13324,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 198,
                                                        end: 200,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 186,
                                            end: 203,
                                            as_str(): "__div(b, a) == 11",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13325,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 179,
                                        end: 185,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31660,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 179,
                            end: 204,
                            as_str(): "assert(__div(b, a) == 11)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 179,
                    end: 204,
                    as_str(): "assert(__div(b, a) == 11)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 213,
                                    end: 214,
                                    as_str(): "a",
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
                                    31661,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 222,
                                    end: 224,
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
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 216,
                                    end: 219,
                                    as_str(): "u16",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 209,
                    end: 225,
                    as_str(): "let a: u16 = 22;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 232,
                                    end: 233,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U16(
                                        44,
                                    ),
                                ),
                                return_type: TypeId(
                                    31662,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 241,
                                    end: 243,
                                    as_str(): "44",
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
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 235,
                                    end: 238,
                                    as_str(): "u16",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 228,
                    end: 244,
                    as_str(): "let b: u16 = 44;",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 266,
                                                            end: 268,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 266,
                                                            end: 268,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 266,
                                                        end: 268,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Add,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 213,
                                                                                    end: 214,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 260,
                                                                                end: 261,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 260,
                                                                            end: 261,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 232,
                                                                                    end: 233,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 263,
                                                                                end: 264,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 263,
                                                                            end: 264,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 254,
                                                                    end: 265,
                                                                    as_str(): "__add(a, b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 254,
                                                            end: 265,
                                                            as_str(): "__add(a, b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            U64(
                                                                66,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 269,
                                                            end: 271,
                                                            as_str(): "66",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13327,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 266,
                                                        end: 268,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 254,
                                            end: 271,
                                            as_str(): "__add(a, b) == 66",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13328,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 247,
                                        end: 253,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31669,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 247,
                            end: 272,
                            as_str(): "assert(__add(a, b) == 66)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 247,
                    end: 272,
                    as_str(): "assert(__add(a, b) == 66)",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 276,
                                        end: 282,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 295,
                                                            end: 297,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 295,
                                                            end: 297,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 295,
                                                        end: 297,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Sub,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 232,
                                                                                    end: 233,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 289,
                                                                                end: 290,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 289,
                                                                            end: 290,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 213,
                                                                                    end: 214,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 292,
                                                                                end: 293,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 292,
                                                                            end: 293,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 283,
                                                                    end: 294,
                                                                    as_str(): "__sub(b, a)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 283,
                                                            end: 294,
                                                            as_str(): "__sub(b, a)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            U64(
                                                                22,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 298,
                                                            end: 300,
                                                            as_str(): "22",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13330,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 295,
                                                        end: 297,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 283,
                                            end: 300,
                                            as_str(): "__sub(b, a) == 22",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13331,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 276,
                                        end: 282,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31676,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 276,
                            end: 301,
                            as_str(): "assert(__sub(b, a) == 22)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 276,
                    end: 301,
                    as_str(): "assert(__sub(b, a) == 22)",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 305,
                                        end: 311,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 324,
                                                            end: 326,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 324,
                                                            end: 326,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 324,
                                                        end: 326,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Mul,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 213,
                                                                                    end: 214,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 318,
                                                                                end: 319,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 318,
                                                                            end: 319,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 232,
                                                                                    end: 233,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 321,
                                                                                end: 322,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 321,
                                                                            end: 322,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 312,
                                                                    end: 323,
                                                                    as_str(): "__mul(a, b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 312,
                                                            end: 323,
                                                            as_str(): "__mul(a, b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            U64(
                                                                968,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 327,
                                                            end: 330,
                                                            as_str(): "968",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13333,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 324,
                                                        end: 326,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 312,
                                            end: 330,
                                            as_str(): "__mul(a, b) == 968",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13334,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 305,
                                        end: 311,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31683,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 305,
                            end: 331,
                            as_str(): "assert(__mul(a, b) == 968)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 305,
                    end: 331,
                    as_str(): "assert(__mul(a, b) == 968)",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 335,
                                        end: 341,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 354,
                                                            end: 356,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 354,
                                                            end: 356,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 354,
                                                        end: 356,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Div,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 232,
                                                                                    end: 233,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 348,
                                                                                end: 349,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 348,
                                                                            end: 349,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 213,
                                                                                    end: 214,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 351,
                                                                                end: 352,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 351,
                                                                            end: 352,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 342,
                                                                    end: 353,
                                                                    as_str(): "__div(b, a)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 342,
                                                            end: 353,
                                                            as_str(): "__div(b, a)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            U64(
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 357,
                                                            end: 358,
                                                            as_str(): "2",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13336,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 354,
                                                        end: 356,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 342,
                                            end: 358,
                                            as_str(): "__div(b, a) == 2",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13337,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 335,
                                        end: 341,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31690,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 335,
                            end: 359,
                            as_str(): "assert(__div(b, a) == 2)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 335,
                    end: 359,
                    as_str(): "assert(__div(b, a) == 2)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 368,
                                    end: 369,
                                    as_str(): "a",
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
                                    31691,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 377,
                                    end: 379,
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
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 371,
                                    end: 374,
                                    as_str(): "u32",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 364,
                    end: 380,
                    as_str(): "let a: u32 = 22;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 387,
                                    end: 388,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U32(
                                        44,
                                    ),
                                ),
                                return_type: TypeId(
                                    31692,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 396,
                                    end: 398,
                                    as_str(): "44",
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
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 390,
                                    end: 393,
                                    as_str(): "u32",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 383,
                    end: 399,
                    as_str(): "let b: u32 = 44;",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 402,
                                        end: 408,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 421,
                                                            end: 423,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 421,
                                                            end: 423,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 421,
                                                        end: 423,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Add,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 368,
                                                                                    end: 369,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 415,
                                                                                end: 416,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 415,
                                                                            end: 416,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 387,
                                                                                    end: 388,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 418,
                                                                                end: 419,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 418,
                                                                            end: 419,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 409,
                                                                    end: 420,
                                                                    as_str(): "__add(a, b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 409,
                                                            end: 420,
                                                            as_str(): "__add(a, b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            U64(
                                                                66,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 424,
                                                            end: 426,
                                                            as_str(): "66",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13339,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 421,
                                                        end: 423,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 409,
                                            end: 426,
                                            as_str(): "__add(a, b) == 66",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13340,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 402,
                                        end: 408,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31699,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 402,
                            end: 427,
                            as_str(): "assert(__add(a, b) == 66)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 402,
                    end: 427,
                    as_str(): "assert(__add(a, b) == 66)",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 431,
                                        end: 437,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 450,
                                                            end: 452,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 450,
                                                            end: 452,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 450,
                                                        end: 452,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Sub,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 387,
                                                                                    end: 388,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 444,
                                                                                end: 445,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 444,
                                                                            end: 445,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 368,
                                                                                    end: 369,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 447,
                                                                                end: 448,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 447,
                                                                            end: 448,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 438,
                                                                    end: 449,
                                                                    as_str(): "__sub(b, a)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 438,
                                                            end: 449,
                                                            as_str(): "__sub(b, a)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            U64(
                                                                22,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 453,
                                                            end: 455,
                                                            as_str(): "22",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13342,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 450,
                                                        end: 452,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 438,
                                            end: 455,
                                            as_str(): "__sub(b, a) == 22",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13343,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 431,
                                        end: 437,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31706,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 431,
                            end: 456,
                            as_str(): "assert(__sub(b, a) == 22)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 431,
                    end: 456,
                    as_str(): "assert(__sub(b, a) == 22)",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 460,
                                        end: 466,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 479,
                                                            end: 481,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 479,
                                                            end: 481,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 479,
                                                        end: 481,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Mul,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 368,
                                                                                    end: 369,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 473,
                                                                                end: 474,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 473,
                                                                            end: 474,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 387,
                                                                                    end: 388,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 476,
                                                                                end: 477,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 476,
                                                                            end: 477,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 467,
                                                                    end: 478,
                                                                    as_str(): "__mul(a, b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 467,
                                                            end: 478,
                                                            as_str(): "__mul(a, b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            U64(
                                                                968,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 482,
                                                            end: 485,
                                                            as_str(): "968",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13345,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 479,
                                                        end: 481,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 467,
                                            end: 485,
                                            as_str(): "__mul(a, b) == 968",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13346,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 460,
                                        end: 466,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31713,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 460,
                            end: 486,
                            as_str(): "assert(__mul(a, b) == 968)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 460,
                    end: 486,
                    as_str(): "assert(__mul(a, b) == 968)",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 490,
                                        end: 496,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 509,
                                                            end: 511,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 509,
                                                            end: 511,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 509,
                                                        end: 511,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Div,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 387,
                                                                                    end: 388,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 503,
                                                                                end: 504,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 503,
                                                                            end: 504,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 368,
                                                                                    end: 369,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 506,
                                                                                end: 507,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 506,
                                                                            end: 507,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 497,
                                                                    end: 508,
                                                                    as_str(): "__div(b, a)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 497,
                                                            end: 508,
                                                            as_str(): "__div(b, a)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            U64(
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 512,
                                                            end: 513,
                                                            as_str(): "2",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13348,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 509,
                                                        end: 511,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 497,
                                            end: 513,
                                            as_str(): "__div(b, a) == 2",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13349,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 490,
                                        end: 496,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31720,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 490,
                            end: 514,
                            as_str(): "assert(__div(b, a) == 2)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 490,
                    end: 514,
                    as_str(): "assert(__div(b, a) == 2)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 523,
                                    end: 524,
                                    as_str(): "a",
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
                                    31721,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 532,
                                    end: 534,
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
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 526,
                                    end: 529,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 519,
                    end: 535,
                    as_str(): "let a: u64 = 22;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 542,
                                    end: 543,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        44,
                                    ),
                                ),
                                return_type: TypeId(
                                    31722,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 551,
                                    end: 553,
                                    as_str(): "44",
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
                                    src (ptr): 0x00007fb14d3784f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                    ),
                                    start: 545,
                                    end: 548,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 538,
                    end: 554,
                    as_str(): "let b: u64 = 44;",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 557,
                                        end: 563,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 576,
                                                            end: 578,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 576,
                                                            end: 578,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 576,
                                                        end: 578,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Add,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 523,
                                                                                    end: 524,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 570,
                                                                                end: 571,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 570,
                                                                            end: 571,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 542,
                                                                                    end: 543,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 573,
                                                                                end: 574,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 573,
                                                                            end: 574,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 564,
                                                                    end: 575,
                                                                    as_str(): "__add(a, b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 564,
                                                            end: 575,
                                                            as_str(): "__add(a, b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                                66,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 579,
                                                            end: 581,
                                                            as_str(): "66",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13351,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 576,
                                                        end: 578,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 564,
                                            end: 581,
                                            as_str(): "__add(a, b) == 66",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13352,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 557,
                                        end: 563,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31729,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 557,
                            end: 582,
                            as_str(): "assert(__add(a, b) == 66)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 557,
                    end: 582,
                    as_str(): "assert(__add(a, b) == 66)",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 586,
                                        end: 592,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 605,
                                                            end: 607,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 605,
                                                            end: 607,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 605,
                                                        end: 607,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Sub,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 542,
                                                                                    end: 543,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 599,
                                                                                end: 600,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 599,
                                                                            end: 600,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 523,
                                                                                    end: 524,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 602,
                                                                                end: 603,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 602,
                                                                            end: 603,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 593,
                                                                    end: 604,
                                                                    as_str(): "__sub(b, a)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 593,
                                                            end: 604,
                                                            as_str(): "__sub(b, a)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 608,
                                                            end: 610,
                                                            as_str(): "22",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13354,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 605,
                                                        end: 607,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 593,
                                            end: 610,
                                            as_str(): "__sub(b, a) == 22",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13355,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 586,
                                        end: 592,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31736,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 586,
                            end: 611,
                            as_str(): "assert(__sub(b, a) == 22)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 586,
                    end: 611,
                    as_str(): "assert(__sub(b, a) == 22)",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 615,
                                        end: 621,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 634,
                                                            end: 636,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 634,
                                                            end: 636,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 634,
                                                        end: 636,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Mul,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 523,
                                                                                    end: 524,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 628,
                                                                                end: 629,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 628,
                                                                            end: 629,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 542,
                                                                                    end: 543,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 631,
                                                                                end: 632,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 631,
                                                                            end: 632,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 622,
                                                                    end: 633,
                                                                    as_str(): "__mul(a, b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 622,
                                                            end: 633,
                                                            as_str(): "__mul(a, b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                                968,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 637,
                                                            end: 640,
                                                            as_str(): "968",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13357,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 634,
                                                        end: 636,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 622,
                                            end: 640,
                                            as_str(): "__mul(a, b) == 968",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13358,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 615,
                                        end: 621,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31743,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 615,
                            end: 641,
                            as_str(): "assert(__mul(a, b) == 968)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 615,
                    end: 641,
                    as_str(): "assert(__mul(a, b) == 968)",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 645,
                                        end: 651,
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
                                            src (ptr): 0x00007fb13a267b10,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 664,
                                                            end: 666,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 664,
                                                            end: 666,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 664,
                                                        end: 666,
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
                                                            src (ptr): 0x00007fb136385f30,
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
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: Div,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 542,
                                                                                    end: 543,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 658,
                                                                                end: 659,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 658,
                                                                            end: 659,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 523,
                                                                                    end: 524,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 661,
                                                                                end: 662,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 661,
                                                                            end: 662,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 652,
                                                                    end: 663,
                                                                    as_str(): "__div(b, a)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 652,
                                                            end: 663,
                                                            as_str(): "__div(b, a)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb136385f30,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 667,
                                                            end: 668,
                                                            as_str(): "2",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13360,
                                                Span {
                                                    src (ptr): 0x00007fb136385f30,
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
                                                        src (ptr): 0x00007fb14d3784f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                        ),
                                                        start: 664,
                                                        end: 666,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 652,
                                            end: 668,
                                            as_str(): "__div(b, a) == 2",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13361,
                                Span {
                                    src (ptr): 0x00007fb13a267b10,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 645,
                                        end: 651,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31750,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 645,
                            end: 669,
                            as_str(): "assert(__div(b, a) == 2)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 645,
                    end: 669,
                    as_str(): "assert(__div(b, a) == 2)",
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
                            31751,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 675,
                            end: 676,
                            as_str(): "2",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14d3784f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                    ),
                    start: 675,
                    end: 676,
                    as_str(): "2",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb14d3784f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
        ),
        start: 35,
        end: 678,
        as_str(): "fn main() -> u64 {\n\n  let a: u8 = 2;\n  let b: u8 = 22;\n  assert(__add(a, b) == 24);\n  assert(__sub(b, a) == 20);\n  assert(__mul(a, b) == 44);\n  assert(__div(b, a) == 11);\n\n  let a: u16 = 22;\n  let b: u16 = 44;\n  assert(__add(a, b) == 66);\n  assert(__sub(b, a) == 22);\n  assert(__mul(a, b) == 968);\n  assert(__div(b, a) == 2);\n\n  let a: u32 = 22;\n  let b: u32 = 44;\n  assert(__add(a, b) == 66);\n  assert(__sub(b, a) == 22);\n  assert(__mul(a, b) == 968);\n  assert(__div(b, a) == 2);\n\n  let a: u64 = 22;\n  let b: u64 = 44;\n  assert(__add(a, b) == 66);\n  assert(__sub(b, a) == 22);\n  assert(__mul(a, b) == 968);\n  assert(__div(b, a) == 2);\n\n\n  2\n}",
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
        src (ptr): 0x00007fb14d3784f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
        ),
        start: 48,
        end: 51,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

