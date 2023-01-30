
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a0e6287a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 67,
                                    end: 74,
                                    as_str(): "counter",
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
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 77,
                                    end: 78,
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
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 59,
                    end: 79,
                    as_str(): "let mut counter = 0;",
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
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 131,
                                                    end: 132,
                                                    as_str(): "<",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 131,
                                                    end: 132,
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
                                                src (ptr): 0x00007f8a0e6287a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                ),
                                                start: 131,
                                                end: 132,
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
                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 67,
                                                            end: 74,
                                                            as_str(): "counter",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 123,
                                                        end: 130,
                                                        as_str(): "counter",
                                                    },
                                                    mutability: Mutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 123,
                                                    end: 130,
                                                    as_str(): "counter",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 133,
                                                    end: 135,
                                                    as_str(): "10",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13314,
                                        Span {
                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                src (ptr): 0x00007f8a0e6287a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                ),
                                                start: 131,
                                                end: 132,
                                                as_str(): "<",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 123,
                                    end: 135,
                                    as_str(): "counter < 10",
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 146,
                                                                end: 153,
                                                                as_str(): "counter",
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
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 164,
                                                                                end: 165,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 164,
                                                                                end: 165,
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 164,
                                                                            end: 165,
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
                                                                                src (ptr): 0x00007f8a1d2468c0,
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
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 67,
                                                                                        end: 74,
                                                                                        as_str(): "counter",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 156,
                                                                                    end: 163,
                                                                                    as_str(): "counter",
                                                                                },
                                                                                mutability: Mutable,
                                                                            },
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 156,
                                                                                end: 163,
                                                                                as_str(): "counter",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d2468c0,
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
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 166,
                                                                                end: 167,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                function_decl_id: DeclId(
                                                                    13315,
                                                                    Span {
                                                                        src (ptr): 0x00007f8a1d2468c0,
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 164,
                                                                            end: 165,
                                                                            as_str(): "+",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 156,
                                                                end: 167,
                                                                as_str(): "counter + 1",
                                                            },
                                                        },
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    31644,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 146,
                                                    end: 167,
                                                    as_str(): "counter = counter + 1",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 146,
                                            end: 167,
                                            as_str(): "counter = counter + 1",
                                        },
                                    },
                                ],
                            },
                        },
                        return_type: TypeId(
                            31646,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                            ),
                            start: 117,
                            end: 174,
                            as_str(): "while counter < 10 {\n        counter = counter + 1;\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 117,
                    end: 174,
                    as_str(): "while counter < 10 {\n        counter = counter + 1;\n    }",
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
                                        src (ptr): 0x00007f8a0e6287a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                                            src (ptr): 0x00007f8a15cca580,
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
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 196,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 196,
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
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 194,
                                                        end: 196,
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
                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 67,
                                                                    end: 74,
                                                                    as_str(): "counter",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 186,
                                                                end: 193,
                                                                as_str(): "counter",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 186,
                                                            end: 193,
                                                            as_str(): "counter",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                10,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 197,
                                                            end: 199,
                                                            as_str(): "10",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13317,
                                                Span {
                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 194,
                                                        end: 196,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 186,
                                            end: 199,
                                            as_str(): "counter == 10",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13318,
                                Span {
                                    src (ptr): 0x00007f8a15cca580,
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
                                        src (ptr): 0x00007f8a0e6287a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                        ),
                                        start: 179,
                                        end: 185,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31652,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                            ),
                            start: 179,
                            end: 200,
                            as_str(): "assert(counter == 10)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 179,
                    end: 200,
                    as_str(): "assert(counter == 10)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 301,
                                    end: 310,
                                    as_str(): "counter_2",
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
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 313,
                                    end: 314,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31653,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 293,
                    end: 315,
                    as_str(): "let mut counter_2 = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 328,
                                    end: 337,
                                    as_str(): "counter_3",
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
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 340,
                                    end: 341,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31655,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 320,
                    end: 342,
                    as_str(): "let mut counter_3 = 0;",
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
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 363,
                                                    end: 364,
                                                    as_str(): "<",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 363,
                                                    end: 364,
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
                                                src (ptr): 0x00007f8a0e6287a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                ),
                                                start: 363,
                                                end: 364,
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
                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 301,
                                                            end: 310,
                                                            as_str(): "counter_2",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 353,
                                                        end: 362,
                                                        as_str(): "counter_2",
                                                    },
                                                    mutability: Mutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 353,
                                                    end: 362,
                                                    as_str(): "counter_2",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 365,
                                                    end: 367,
                                                    as_str(): "10",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13319,
                                        Span {
                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                src (ptr): 0x00007f8a0e6287a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                ),
                                                start: 363,
                                                end: 364,
                                                as_str(): "<",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 353,
                                    end: 367,
                                    as_str(): "counter_2 < 10",
                                },
                            },
                            body: TyCodeBlock {
                                contents: [
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 391,
                                                                            end: 393,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 391,
                                                                            end: 393,
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
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 391,
                                                                        end: 393,
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
                                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 301,
                                                                                    end: 310,
                                                                                    as_str(): "counter_2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 381,
                                                                                end: 390,
                                                                                as_str(): "counter_2",
                                                                            },
                                                                            mutability: Mutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 381,
                                                                            end: 390,
                                                                            as_str(): "counter_2",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 394,
                                                                            end: 395,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13320,
                                                                Span {
                                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 391,
                                                                        end: 393,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 381,
                                                            end: 395,
                                                            as_str(): "counter_2 == 3",
                                                        },
                                                    },
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: Expression(
                                                                            TyExpression {
                                                                                expression: Reassignment(
                                                                                    TyReassignment {
                                                                                        lhs_base_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 463,
                                                                                                end: 472,
                                                                                                as_str(): "counter_2",
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
                                                                                                    10,
                                                                                                ),
                                                                                            ),
                                                                                            return_type: TypeId(
                                                                                                31668,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 475,
                                                                                                end: 477,
                                                                                                as_str(): "10",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    31670,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 463,
                                                                                    end: 477,
                                                                                    as_str(): "counter_2 = 10",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 463,
                                                                            end: 477,
                                                                            as_str(): "counter_2 = 10",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31672,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 396,
                                                            end: 488,
                                                            as_str(): "{\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        }",
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
                                                                                    expression: Reassignment(
                                                                                        TyReassignment {
                                                                                            lhs_base_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 508,
                                                                                                    end: 517,
                                                                                                    as_str(): "counter_2",
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
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 530,
                                                                                                                    end: 531,
                                                                                                                    as_str(): "+",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "ops",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 530,
                                                                                                                    end: 531,
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
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 530,
                                                                                                                end: 531,
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
                                                                                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 301,
                                                                                                                            end: 310,
                                                                                                                            as_str(): "counter_2",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 520,
                                                                                                                        end: 529,
                                                                                                                        as_str(): "counter_2",
                                                                                                                    },
                                                                                                                    mutability: Mutable,
                                                                                                                },
                                                                                                                return_type: TypeId(
                                                                                                                    21,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 520,
                                                                                                                    end: 529,
                                                                                                                    as_str(): "counter_2",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        (
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 532,
                                                                                                                    end: 533,
                                                                                                                    as_str(): "1",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ],
                                                                                                    function_decl_id: DeclId(
                                                                                                        13321,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 530,
                                                                                                                end: 531,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 520,
                                                                                                    end: 533,
                                                                                                    as_str(): "counter_2 + 1",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    return_type: TypeId(
                                                                                        31680,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 508,
                                                                                        end: 533,
                                                                                        as_str(): "counter_2 = counter_2 + 1",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 508,
                                                                                end: 533,
                                                                                as_str(): "counter_2 = counter_2 + 1",
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
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 547,
                                                                                                    end: 556,
                                                                                                    as_str(): "counter_3",
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
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 569,
                                                                                                                    end: 570,
                                                                                                                    as_str(): "+",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "ops",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 569,
                                                                                                                    end: 570,
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
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 569,
                                                                                                                end: 570,
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
                                                                                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 328,
                                                                                                                            end: 337,
                                                                                                                            as_str(): "counter_3",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 559,
                                                                                                                        end: 568,
                                                                                                                        as_str(): "counter_3",
                                                                                                                    },
                                                                                                                    mutability: Mutable,
                                                                                                                },
                                                                                                                return_type: TypeId(
                                                                                                                    21,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 559,
                                                                                                                    end: 568,
                                                                                                                    as_str(): "counter_3",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        (
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 571,
                                                                                                                    end: 572,
                                                                                                                    as_str(): "1",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ],
                                                                                                    function_decl_id: DeclId(
                                                                                                        13322,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 569,
                                                                                                                end: 570,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 559,
                                                                                                    end: 572,
                                                                                                    as_str(): "counter_3 + 1",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    return_type: TypeId(
                                                                                        31687,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 547,
                                                                                        end: 572,
                                                                                        as_str(): "counter_3 = counter_3 + 1",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 547,
                                                                                end: 572,
                                                                                as_str(): "counter_3 = counter_3 + 1",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                31689,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 494,
                                                                end: 583,
                                                                as_str(): "{\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    31690,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 378,
                                                    end: 583,
                                                    as_str(): "if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 378,
                                            end: 583,
                                            as_str(): "if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }",
                                        },
                                    },
                                ],
                            },
                        },
                        return_type: TypeId(
                            31691,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                            ),
                            start: 347,
                            end: 589,
                            as_str(): "while counter_2 < 10 {\n        if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 347,
                    end: 589,
                    as_str(): "while counter_2 < 10 {\n        if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }\n    }",
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
                                        src (ptr): 0x00007f8a0e6287a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                        ),
                                        start: 595,
                                        end: 601,
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
                                            src (ptr): 0x00007f8a15cca580,
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
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 301,
                                                                            end: 310,
                                                                            as_str(): "counter_2",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 602,
                                                                        end: 611,
                                                                        as_str(): "counter_2",
                                                                    },
                                                                    mutability: Mutable,
                                                                },
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 602,
                                                                    end: 611,
                                                                    as_str(): "counter_2",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                                        10,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 615,
                                                                    end: 617,
                                                                    as_str(): "10",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13324,
                                                        Span {
                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 602,
                                                    end: 617,
                                                    as_str(): "counter_2 == 10",
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
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 631,
                                                                    end: 633,
                                                                    as_str(): "==",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 631,
                                                                    end: 633,
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 631,
                                                                end: 633,
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
                                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 328,
                                                                            end: 337,
                                                                            as_str(): "counter_3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 621,
                                                                        end: 630,
                                                                        as_str(): "counter_3",
                                                                    },
                                                                    mutability: Mutable,
                                                                },
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 621,
                                                                    end: 630,
                                                                    as_str(): "counter_3",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 634,
                                                                    end: 635,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13325,
                                                        Span {
                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 631,
                                                                end: 633,
                                                                as_str(): "==",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 621,
                                                    end: 635,
                                                    as_str(): "counter_3 == 3",
                                                },
                                            },
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 602,
                                            end: 635,
                                            as_str(): "counter_2 == 10 && counter_3 == 3",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13326,
                                Span {
                                    src (ptr): 0x00007f8a15cca580,
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
                                        src (ptr): 0x00007f8a0e6287a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                        ),
                                        start: 595,
                                        end: 601,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31700,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                            ),
                            start: 595,
                            end: 636,
                            as_str(): "assert(counter_2 == 10 && counter_3 == 3)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 595,
                    end: 636,
                    as_str(): "assert(counter_2 == 10 && counter_3 == 3)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 677,
                                    end: 686,
                                    as_str(): "counter_4",
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
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 689,
                                    end: 690,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31701,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 669,
                    end: 691,
                    as_str(): "let mut counter_4 = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 704,
                                    end: 713,
                                    as_str(): "counter_5",
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
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 716,
                                    end: 717,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31703,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 696,
                    end: 718,
                    as_str(): "let mut counter_5 = 0;",
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
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 740,
                                                    end: 741,
                                                    as_str(): "<",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 740,
                                                    end: 741,
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
                                                src (ptr): 0x00007f8a0e6287a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                ),
                                                start: 740,
                                                end: 741,
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
                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 677,
                                                            end: 686,
                                                            as_str(): "counter_4",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 730,
                                                        end: 739,
                                                        as_str(): "counter_4",
                                                    },
                                                    mutability: Mutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 730,
                                                    end: 739,
                                                    as_str(): "counter_4",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                        7,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 742,
                                                    end: 743,
                                                    as_str(): "7",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13327,
                                        Span {
                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                src (ptr): 0x00007f8a0e6287a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                ),
                                                start: 740,
                                                end: 741,
                                                as_str(): "<",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 730,
                                    end: 743,
                                    as_str(): "counter_4 < 7",
                                },
                            },
                            body: TyCodeBlock {
                                contents: [
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 770,
                                                                            end: 771,
                                                                            as_str(): "<",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 770,
                                                                            end: 771,
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
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 770,
                                                                        end: 771,
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
                                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 704,
                                                                                    end: 713,
                                                                                    as_str(): "counter_5",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 760,
                                                                                end: 769,
                                                                                as_str(): "counter_5",
                                                                            },
                                                                            mutability: Mutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 760,
                                                                            end: 769,
                                                                            as_str(): "counter_5",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                                11,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 772,
                                                                            end: 774,
                                                                            as_str(): "11",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13328,
                                                                Span {
                                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 770,
                                                                        end: 771,
                                                                        as_str(): "<",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 760,
                                                            end: 774,
                                                            as_str(): "counter_5 < 11",
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
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 789,
                                                                                        end: 798,
                                                                                        as_str(): "counter_5",
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
                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 811,
                                                                                                        end: 812,
                                                                                                        as_str(): "+",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 811,
                                                                                                        end: 812,
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
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 811,
                                                                                                    end: 812,
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
                                                                                                        src (ptr): 0x00007f8a1d2468c0,
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
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 704,
                                                                                                                end: 713,
                                                                                                                as_str(): "counter_5",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 801,
                                                                                                            end: 810,
                                                                                                            as_str(): "counter_5",
                                                                                                        },
                                                                                                        mutability: Mutable,
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 801,
                                                                                                        end: 810,
                                                                                                        as_str(): "counter_5",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a1d2468c0,
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
                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 813,
                                                                                                        end: 814,
                                                                                                        as_str(): "1",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            13329,
                                                                                            Span {
                                                                                                src (ptr): 0x00007f8a1d2468c0,
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
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 811,
                                                                                                    end: 812,
                                                                                                    as_str(): "+",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 801,
                                                                                        end: 814,
                                                                                        as_str(): "counter_5 + 1",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31721,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 789,
                                                                            end: 814,
                                                                            as_str(): "counter_5 = counter_5 + 1",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 789,
                                                                    end: 814,
                                                                    as_str(): "counter_5 = counter_5 + 1",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                },
                                                return_type: TypeId(
                                                    31723,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 754,
                                                    end: 825,
                                                    as_str(): "while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 754,
                                            end: 825,
                                            as_str(): "while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }",
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 834,
                                                                end: 843,
                                                                as_str(): "counter_4",
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
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 856,
                                                                                end: 857,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 856,
                                                                                end: 857,
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 856,
                                                                            end: 857,
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
                                                                                src (ptr): 0x00007f8a1d2468c0,
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
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 677,
                                                                                        end: 686,
                                                                                        as_str(): "counter_4",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 846,
                                                                                    end: 855,
                                                                                    as_str(): "counter_4",
                                                                                },
                                                                                mutability: Mutable,
                                                                            },
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 846,
                                                                                end: 855,
                                                                                as_str(): "counter_4",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d2468c0,
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
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 858,
                                                                                end: 859,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                function_decl_id: DeclId(
                                                                    13330,
                                                                    Span {
                                                                        src (ptr): 0x00007f8a1d2468c0,
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 856,
                                                                            end: 857,
                                                                            as_str(): "+",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 846,
                                                                end: 859,
                                                                as_str(): "counter_4 + 1",
                                                            },
                                                        },
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    31730,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 834,
                                                    end: 859,
                                                    as_str(): "counter_4 = counter_4 + 1",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 834,
                                            end: 859,
                                            as_str(): "counter_4 = counter_4 + 1",
                                        },
                                    },
                                ],
                            },
                        },
                        return_type: TypeId(
                            31732,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                            ),
                            start: 724,
                            end: 866,
                            as_str(): "while counter_4 < 7 {\n        while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }\n        counter_4 = counter_4 + 1;\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 724,
                    end: 866,
                    as_str(): "while counter_4 < 7 {\n        while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }\n        counter_4 = counter_4 + 1;\n    }",
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
                                        src (ptr): 0x00007f8a0e6287a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                        ),
                                        start: 871,
                                        end: 877,
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
                                            src (ptr): 0x00007f8a15cca580,
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
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 888,
                                                            end: 890,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 888,
                                                            end: 890,
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
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 888,
                                                        end: 890,
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
                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 704,
                                                                    end: 713,
                                                                    as_str(): "counter_5",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 878,
                                                                end: 887,
                                                                as_str(): "counter_5",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 878,
                                                            end: 887,
                                                            as_str(): "counter_5",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                11,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 891,
                                                            end: 893,
                                                            as_str(): "11",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13332,
                                                Span {
                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 888,
                                                        end: 890,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 878,
                                            end: 893,
                                            as_str(): "counter_5 == 11",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13333,
                                Span {
                                    src (ptr): 0x00007f8a15cca580,
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
                                        src (ptr): 0x00007f8a0e6287a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                        ),
                                        start: 871,
                                        end: 877,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31738,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                            ),
                            start: 871,
                            end: 894,
                            as_str(): "assert(counter_5 == 11)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 871,
                    end: 894,
                    as_str(): "assert(counter_5 == 11)",
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
                                        src (ptr): 0x00007f8a0e6287a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                        ),
                                        start: 900,
                                        end: 906,
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
                                            src (ptr): 0x00007f8a15cca580,
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
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 917,
                                                            end: 919,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 917,
                                                            end: 919,
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
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 917,
                                                        end: 919,
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
                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 677,
                                                                    end: 686,
                                                                    as_str(): "counter_4",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 907,
                                                                end: 916,
                                                                as_str(): "counter_4",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 907,
                                                            end: 916,
                                                            as_str(): "counter_4",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d2468c0,
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
                                                                7,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 920,
                                                            end: 921,
                                                            as_str(): "7",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13335,
                                                Span {
                                                    src (ptr): 0x00007f8a1d2468c0,
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
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 917,
                                                        end: 919,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 907,
                                            end: 921,
                                            as_str(): "counter_4 == 7",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13336,
                                Span {
                                    src (ptr): 0x00007f8a15cca580,
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
                                        src (ptr): 0x00007f8a0e6287a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                        ),
                                        start: 900,
                                        end: 906,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31744,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                            ),
                            start: 900,
                            end: 922,
                            as_str(): "assert(counter_4 == 7)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 900,
                    end: 922,
                    as_str(): "assert(counter_4 == 7)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 967,
                                    end: 973,
                                    as_str(): "result",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
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
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 982,
                                            end: 986,
                                            as_str(): "true",
                                        },
                                    },
                                    body: TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Expression(
                                                    TyExpression {
                                                        expression: Break,
                                                        return_type: TypeId(
                                                            31748,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 989,
                                                            end: 994,
                                                            as_str(): "break",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 989,
                                                    end: 994,
                                                    as_str(): "break",
                                                },
                                            },
                                        ],
                                    },
                                },
                                return_type: TypeId(
                                    31750,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 976,
                                    end: 997,
                                    as_str(): "while true { break; }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31750,
                            ),
                            type_ascription: TypeId(
                                31745,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 963,
                    end: 998,
                    as_str(): "let result = while true { break; };",
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
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                            ),
                            start: 1004,
                            end: 1008,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a0e6287a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                    ),
                    start: 1004,
                    end: 1008,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a0e6287a0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
        ),
        start: 35,
        end: 1010,
        as_str(): "fn main() -> bool {\n    let mut counter = 0;\n    // test standard while loop:\n    while counter < 10 {\n        counter = counter + 1;\n    }\n    assert(counter == 10);\n\n    // test early exit from loop with manual \"break\" (by invalidating the condition):\n    let mut counter_2 = 0;\n    let mut counter_3 = 0;\n    while counter_2 < 10 {\n        if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }\n    }\n\n    assert(counter_2 == 10 && counter_3 == 3);\n\n    // test nested loops:\n    let mut counter_4 = 0;\n    let mut counter_5 = 0;\n\n    while counter_4 < 7 {\n        while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }\n        counter_4 = counter_4 + 1;\n    }\n    assert(counter_5 == 11);\n    assert(counter_4 == 7);\n\n    // test while loop expression\n    let result = while true { break; };\n\n    true\n}",
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
        src (ptr): 0x00007f8a0e6287a0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
        ),
        start: 48,
        end: 52,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

