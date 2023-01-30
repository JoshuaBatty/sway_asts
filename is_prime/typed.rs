

TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd209f60,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
            ),
            start: 51,
            end: 62,
            as_str(): "check_prime",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: LazyOperator {
                                    op: Or,
                                    lhs: TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 63,
                                                                    end: 64,
                                                                    as_str(): "n",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 88,
                                                                end: 89,
                                                                as_str(): "n",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 88,
                                                            end: 89,
                                                            as_str(): "n",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 93,
                                                            end: 94,
                                                            as_str(): "0",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13314,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 88,
                                            end: 94,
                                            as_str(): "n == 0",
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 100,
                                                            end: 102,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 100,
                                                            end: 102,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 100,
                                                        end: 102,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 63,
                                                                    end: 64,
                                                                    as_str(): "n",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 98,
                                                                end: 99,
                                                                as_str(): "n",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 99,
                                                            as_str(): "n",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 103,
                                                            end: 104,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13315,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 100,
                                                        end: 102,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 98,
                                            end: 104,
                                            as_str(): "n == 1",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 88,
                                    end: 104,
                                    as_str(): "n == 0 || n == 1",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 115,
                                                            end: 120,
                                                            as_str(): "false",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd209f60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                    ),
                                                    start: 115,
                                                    end: 120,
                                                    as_str(): "false",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 105,
                                    end: 126,
                                    as_str(): "{\n        false\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: Declaration(
                                                        VariableDeclaration(
                                                            TyVariableDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 150,
                                                                        end: 162,
                                                                        as_str(): "is_not_prime",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                body: TyExpression {
                                                                    expression: Literal(
                                                                        Boolean(
                                                                            false,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 165,
                                                                        end: 170,
                                                                        as_str(): "false",
                                                                    },
                                                                },
                                                                mutability: Mutable,
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                type_ascription: TypeId(
                                                                    31639,
                                                                ),
                                                                type_ascription_span: None,
                                                            },
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 142,
                                                        end: 171,
                                                        as_str(): "let mut is_not_prime = false;",
                                                    },
                                                },
                                                TyAstNode {
                                                    content: Declaration(
                                                        VariableDeclaration(
                                                            TyVariableDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 188,
                                                                        end: 189,
                                                                        as_str(): "i",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                body: TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 192,
                                                                        end: 193,
                                                                        as_str(): "2",
                                                                    },
                                                                },
                                                                mutability: Mutable,
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                type_ascription: TypeId(
                                                                    31640,
                                                                ),
                                                                type_ascription_span: None,
                                                            },
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 180,
                                                        end: 194,
                                                        as_str(): "let mut i = 2;",
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 211,
                                                                                        end: 212,
                                                                                        as_str(): "<",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 211,
                                                                                        end: 212,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 211,
                                                                                    end: 212,
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
                                                                                        src (ptr): 0x00007fe0d3d29400,
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
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 188,
                                                                                                end: 189,
                                                                                                as_str(): "i",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 209,
                                                                                            end: 210,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                        mutability: Mutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 209,
                                                                                        end: 210,
                                                                                        as_str(): "i",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d3d29400,
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
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 63,
                                                                                                end: 64,
                                                                                                as_str(): "n",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 213,
                                                                                            end: 214,
                                                                                            as_str(): "n",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 213,
                                                                                        end: 214,
                                                                                        as_str(): "n",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        function_decl_id: DeclId(
                                                                            13316,
                                                                            Span {
                                                                                src (ptr): 0x00007fe0d3d29400,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 211,
                                                                                    end: 212,
                                                                                    as_str(): "<",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    return_type: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 209,
                                                                        end: 214,
                                                                        as_str(): "i < n",
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
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                                                                                                src (ptr): 0x00007fe0d3d29400,
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
                                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 235,
                                                                                                                                end: 236,
                                                                                                                                as_str(): "%",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: Some(
                                                                                                                                "ops",
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 235,
                                                                                                                                end: 236,
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
                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 235,
                                                                                                                            end: 236,
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
                                                                                                                                src (ptr): 0x00007fe0d3d29400,
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
                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 63,
                                                                                                                                        end: 64,
                                                                                                                                        as_str(): "n",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 233,
                                                                                                                                    end: 234,
                                                                                                                                    as_str(): "n",
                                                                                                                                },
                                                                                                                                mutability: Immutable,
                                                                                                                            },
                                                                                                                            return_type: TypeId(
                                                                                                                                21,
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 233,
                                                                                                                                end: 234,
                                                                                                                                as_str(): "n",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    (
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0d3d29400,
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
                                                                                                                            expression: VariableExpression {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 188,
                                                                                                                                        end: 189,
                                                                                                                                        as_str(): "i",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 237,
                                                                                                                                    end: 238,
                                                                                                                                    as_str(): "i",
                                                                                                                                },
                                                                                                                                mutability: Mutable,
                                                                                                                            },
                                                                                                                            return_type: TypeId(
                                                                                                                                21,
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 237,
                                                                                                                                end: 238,
                                                                                                                                as_str(): "i",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ],
                                                                                                                function_decl_id: DeclId(
                                                                                                                    13317,
                                                                                                                    Span {
                                                                                                                        src (ptr): 0x00007fe0d3d29400,
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
                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 235,
                                                                                                                            end: 236,
                                                                                                                            as_str(): "%",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                21,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 233,
                                                                                                                end: 238,
                                                                                                                as_str(): "n % i",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    (
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d3d29400,
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
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 242,
                                                                                                                end: 243,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                                function_decl_id: DeclId(
                                                                                                    13318,
                                                                                                    Span {
                                                                                                        src (ptr): 0x00007fe0d3d29400,
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
                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 233,
                                                                                                end: 243,
                                                                                                as_str(): "n % i == 0",
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
                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 262,
                                                                                                                                    end: 274,
                                                                                                                                    as_str(): "is_not_prime",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            lhs_type: TypeId(
                                                                                                                                71,
                                                                                                                            ),
                                                                                                                            lhs_indices: [],
                                                                                                                            rhs: TyExpression {
                                                                                                                                expression: Literal(
                                                                                                                                    Boolean(
                                                                                                                                        true,
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                                return_type: TypeId(
                                                                                                                                    71,
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 277,
                                                                                                                                    end: 281,
                                                                                                                                    as_str(): "true",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    return_type: TypeId(
                                                                                                                        31656,
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 262,
                                                                                                                        end: 281,
                                                                                                                        as_str(): "is_not_prime = true",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 262,
                                                                                                                end: 281,
                                                                                                                as_str(): "is_not_prime = true",
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
                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 299,
                                                                                                                                    end: 300,
                                                                                                                                    as_str(): "i",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            lhs_type: TypeId(
                                                                                                                                21,
                                                                                                                            ),
                                                                                                                            lhs_indices: [],
                                                                                                                            rhs: TyExpression {
                                                                                                                                expression: VariableExpression {
                                                                                                                                    name: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 63,
                                                                                                                                            end: 64,
                                                                                                                                            as_str(): "n",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 303,
                                                                                                                                        end: 304,
                                                                                                                                        as_str(): "n",
                                                                                                                                    },
                                                                                                                                    mutability: Immutable,
                                                                                                                                },
                                                                                                                                return_type: TypeId(
                                                                                                                                    21,
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 303,
                                                                                                                                    end: 304,
                                                                                                                                    as_str(): "n",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    return_type: TypeId(
                                                                                                                        31660,
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 299,
                                                                                                                        end: 304,
                                                                                                                        as_str(): "i = n",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 299,
                                                                                                                end: 304,
                                                                                                                as_str(): "i = n",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            return_type: TypeId(
                                                                                                31662,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 244,
                                                                                                end: 328,
                                                                                                as_str(): "{\n                is_not_prime = true;\n                i = n; // break\n            }",
                                                                                            },
                                                                                        },
                                                                                        else: None,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        31665,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 230,
                                                                                        end: 328,
                                                                                        as_str(): "if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            }",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 230,
                                                                                end: 328,
                                                                                as_str(): "if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            }",
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
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 342,
                                                                                                    end: 343,
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
                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 348,
                                                                                                                    end: 349,
                                                                                                                    as_str(): "+",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "ops",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 348,
                                                                                                                    end: 349,
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
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 348,
                                                                                                                end: 349,
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
                                                                                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 188,
                                                                                                                            end: 189,
                                                                                                                            as_str(): "i",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 346,
                                                                                                                        end: 347,
                                                                                                                        as_str(): "i",
                                                                                                                    },
                                                                                                                    mutability: Mutable,
                                                                                                                },
                                                                                                                return_type: TypeId(
                                                                                                                    21,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 346,
                                                                                                                    end: 347,
                                                                                                                    as_str(): "i",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        (
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 350,
                                                                                                                    end: 351,
                                                                                                                    as_str(): "1",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ],
                                                                                                    function_decl_id: DeclId(
                                                                                                        13319,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 348,
                                                                                                                end: 349,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 346,
                                                                                                    end: 351,
                                                                                                    as_str(): "i + 1",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    return_type: TypeId(
                                                                                        31672,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 342,
                                                                                        end: 351,
                                                                                        as_str(): "i = i + 1",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 342,
                                                                                end: 351,
                                                                                as_str(): "i = i + 1",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                            return_type: TypeId(
                                                                31674,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 203,
                                                                end: 362,
                                                                as_str(): "while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 203,
                                                        end: 362,
                                                        as_str(): "while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }",
                                                    },
                                                },
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
                                                        TyExpression {
                                                            expression: FunctionApplication {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 372,
                                                                                end: 373,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 372,
                                                                                end: 373,
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 372,
                                                                            end: 373,
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
                                                                                src (ptr): 0x00007fe0d3d29400,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 2713,
                                                                                end: 2717,
                                                                                as_str(): "self",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        TyExpression {
                                                                            expression: VariableExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 150,
                                                                                        end: 162,
                                                                                        as_str(): "is_not_prime",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 373,
                                                                                    end: 385,
                                                                                    as_str(): "is_not_prime",
                                                                                },
                                                                                mutability: Mutable,
                                                                            },
                                                                            return_type: TypeId(
                                                                                71,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 373,
                                                                                end: 385,
                                                                                as_str(): "is_not_prime",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                function_decl_id: DeclId(
                                                                    13320,
                                                                    Span {
                                                                        src (ptr): 0x00007fe0d3d29400,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                        ),
                                                                        start: 2706,
                                                                        end: 2760,
                                                                        as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                                    },
                                                                ),
                                                                self_state_idx: None,
                                                                selector: None,
                                                                type_binding: Some(
                                                                    TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 372,
                                                                            end: 373,
                                                                            as_str(): "!",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                71,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 372,
                                                                end: 385,
                                                                as_str(): "!is_not_prime",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 372,
                                                        end: 385,
                                                        as_str(): "!is_not_prime",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        71,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 132,
                                        end: 391,
                                        as_str(): "{\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 85,
                            end: 391,
                            as_str(): "if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 85,
                    end: 391,
                    as_str(): "if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 63,
                    end: 64,
                    as_str(): "n",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fd209f60,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                ),
                start: 66,
                end: 69,
                as_str(): "u64",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fd209f60,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
        ),
        start: 48,
        end: 393,
        as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
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
        src (ptr): 0x00007fe0fd209f60,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
        ),
        start: 74,
        end: 78,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd209f60,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
            ),
            start: 398,
            end: 402,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 419,
                                        end: 425,
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 442,
                                                            end: 444,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 442,
                                                            end: 444,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 442,
                                                        end: 444,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 426,
                                                                        end: 437,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                64,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 438,
                                                                            end: 440,
                                                                            as_str(): "64",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13324,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 426,
                                                                        end: 437,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 426,
                                                            end: 441,
                                                            as_str(): "check_prime(64)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                                false,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 445,
                                                            end: 450,
                                                            as_str(): "false",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13325,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 442,
                                                        end: 444,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 426,
                                            end: 450,
                                            as_str(): "check_prime(64) == false",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13326,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 419,
                                        end: 425,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31683,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 419,
                            end: 451,
                            as_str(): "assert(check_prime(64) == false)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 419,
                    end: 451,
                    as_str(): "assert(check_prime(64) == false)",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 457,
                                        end: 463,
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 464,
                                                                        end: 475,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                8,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 476,
                                                                            end: 477,
                                                                            as_str(): "8",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13329,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 464,
                                                                        end: 475,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 464,
                                                            end: 478,
                                                            as_str(): "check_prime(8)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                                false,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 482,
                                                            end: 487,
                                                            as_str(): "false",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13330,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 464,
                                            end: 487,
                                            as_str(): "check_prime(8) == false",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13331,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 457,
                                        end: 463,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31690,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 457,
                            end: 488,
                            as_str(): "assert(check_prime(8) == false)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 457,
                    end: 488,
                    as_str(): "assert(check_prime(8) == false)",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 494,
                                        end: 500,
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 516,
                                                            end: 518,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 516,
                                                            end: 518,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 516,
                                                        end: 518,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 501,
                                                                        end: 512,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 513,
                                                                            end: 514,
                                                                            as_str(): "7",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13334,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 501,
                                                                        end: 512,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 501,
                                                            end: 515,
                                                            as_str(): "check_prime(7)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 519,
                                                            end: 523,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13335,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 516,
                                                        end: 518,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 501,
                                            end: 523,
                                            as_str(): "check_prime(7) == true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13336,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 494,
                                        end: 500,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31697,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 494,
                            end: 524,
                            as_str(): "assert(check_prime(7) == true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 494,
                    end: 524,
                    as_str(): "assert(check_prime(7) == true)",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 530,
                                        end: 536,
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 553,
                                                            end: 555,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 553,
                                                            end: 555,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 553,
                                                        end: 555,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 537,
                                                                        end: 548,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 549,
                                                                            end: 551,
                                                                            as_str(): "11",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13339,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 537,
                                                                        end: 548,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 537,
                                                            end: 552,
                                                            as_str(): "check_prime(11)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 556,
                                                            end: 560,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13340,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 553,
                                                        end: 555,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 537,
                                            end: 560,
                                            as_str(): "check_prime(11) == true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13341,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 530,
                                        end: 536,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31704,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 530,
                            end: 561,
                            as_str(): "assert(check_prime(11) == true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 530,
                    end: 561,
                    as_str(): "assert(check_prime(11) == true)",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 567,
                                        end: 573,
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 590,
                                                            end: 592,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 590,
                                                            end: 592,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 590,
                                                        end: 592,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 574,
                                                                        end: 585,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                13,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 586,
                                                                            end: 588,
                                                                            as_str(): "13",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13344,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 574,
                                                                        end: 585,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 574,
                                                            end: 589,
                                                            as_str(): "check_prime(13)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 593,
                                                            end: 597,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13345,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 590,
                                                        end: 592,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 574,
                                            end: 597,
                                            as_str(): "check_prime(13) == true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13346,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 567,
                                        end: 573,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31711,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 567,
                            end: 598,
                            as_str(): "assert(check_prime(13) == true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 567,
                    end: 598,
                    as_str(): "assert(check_prime(13) == true)",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 626,
                                                            end: 628,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 626,
                                                            end: 628,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 626,
                                                        end: 628,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 611,
                                                                        end: 622,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 623,
                                                                            end: 624,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13349,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 611,
                                                                        end: 622,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 611,
                                                            end: 625,
                                                            as_str(): "check_prime(2)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 629,
                                                            end: 633,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13350,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 626,
                                                        end: 628,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 611,
                                            end: 633,
                                            as_str(): "check_prime(2) == true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13351,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 604,
                                        end: 610,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31718,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 604,
                            end: 634,
                            as_str(): "assert(check_prime(2) == true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 604,
                    end: 634,
                    as_str(): "assert(check_prime(2) == true)",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 640,
                                        end: 646,
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 662,
                                                            end: 664,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 662,
                                                            end: 664,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 662,
                                                        end: 664,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 647,
                                                                        end: 658,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 659,
                                                                            end: 660,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13354,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 647,
                                                                        end: 658,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 647,
                                                            end: 661,
                                                            as_str(): "check_prime(3)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 665,
                                                            end: 669,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13355,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 662,
                                                        end: 664,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 647,
                                            end: 669,
                                            as_str(): "check_prime(3) == true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13356,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 640,
                                        end: 646,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31725,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 640,
                            end: 670,
                            as_str(): "assert(check_prime(3) == true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 640,
                    end: 670,
                    as_str(): "assert(check_prime(3) == true)",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 676,
                                        end: 682,
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 698,
                                                            end: 700,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 698,
                                                            end: 700,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 698,
                                                        end: 700,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 683,
                                                                        end: 694,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 695,
                                                                            end: 696,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13359,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 683,
                                                                        end: 694,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 683,
                                                            end: 697,
                                                            as_str(): "check_prime(1)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                                false,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 701,
                                                            end: 706,
                                                            as_str(): "false",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13360,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 698,
                                                        end: 700,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 683,
                                            end: 706,
                                            as_str(): "check_prime(1) == false",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13361,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 676,
                                        end: 682,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31732,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 676,
                            end: 707,
                            as_str(): "assert(check_prime(1) == false)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 676,
                    end: 707,
                    as_str(): "assert(check_prime(1) == false)",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 713,
                                        end: 719,
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 735,
                                                            end: 737,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 735,
                                                            end: 737,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 735,
                                                        end: 737,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 720,
                                                                        end: 731,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 732,
                                                                            end: 733,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13364,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 720,
                                                                        end: 731,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 720,
                                                            end: 734,
                                                            as_str(): "check_prime(0)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                                false,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 738,
                                                            end: 743,
                                                            as_str(): "false",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13365,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 735,
                                                        end: 737,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 720,
                                            end: 743,
                                            as_str(): "check_prime(0) == false",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13366,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 713,
                                        end: 719,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31739,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 713,
                            end: 744,
                            as_str(): "assert(check_prime(0) == false)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 713,
                    end: 744,
                    as_str(): "assert(check_prime(0) == false)",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 751,
                                        end: 757,
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 774,
                                                            end: 776,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 774,
                                                            end: 776,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 774,
                                                        end: 776,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 758,
                                                                        end: 769,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 770,
                                                                            end: 772,
                                                                            as_str(): "11",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13369,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 758,
                                                                        end: 769,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 758,
                                                            end: 773,
                                                            as_str(): "check_prime(11)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 777,
                                                                        end: 788,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                17,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 789,
                                                                            end: 791,
                                                                            as_str(): "17",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13371,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 777,
                                                                        end: 788,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 777,
                                                            end: 792,
                                                            as_str(): "check_prime(17)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13372,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 774,
                                                        end: 776,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 758,
                                            end: 792,
                                            as_str(): "check_prime(11) == check_prime(17)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13373,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 751,
                                        end: 757,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31748,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 751,
                            end: 793,
                            as_str(): "assert(check_prime(11) == check_prime(17))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 751,
                    end: 793,
                    as_str(): "assert(check_prime(11) == check_prime(17))",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 799,
                                        end: 805,
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 822,
                                                            end: 824,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 822,
                                                            end: 824,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 822,
                                                        end: 824,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 806,
                                                                        end: 817,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                12,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 818,
                                                                            end: 820,
                                                                            as_str(): "12",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13376,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 806,
                                                                        end: 817,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 806,
                                                            end: 821,
                                                            as_str(): "check_prime(12)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                                false,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 825,
                                                            end: 830,
                                                            as_str(): "false",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13377,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 822,
                                                        end: 824,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 806,
                                            end: 830,
                                            as_str(): "check_prime(12) == false",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13378,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 799,
                                        end: 805,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31755,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 799,
                            end: 831,
                            as_str(): "assert(check_prime(12) == false)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 799,
                    end: 831,
                    as_str(): "assert(check_prime(12) == false)",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 837,
                                        end: 843,
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 860,
                                                            end: 862,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 860,
                                                            end: 862,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 860,
                                                        end: 862,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 844,
                                                                        end: 855,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                18,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 856,
                                                                            end: 858,
                                                                            as_str(): "18",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13381,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 844,
                                                                        end: 855,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 844,
                                                            end: 859,
                                                            as_str(): "check_prime(18)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                                false,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 863,
                                                            end: 868,
                                                            as_str(): "false",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13382,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 860,
                                                        end: 862,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 844,
                                            end: 868,
                                            as_str(): "check_prime(18) == false",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13383,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 837,
                                        end: 843,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31762,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 837,
                            end: 869,
                            as_str(): "assert(check_prime(18) == false)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 837,
                    end: 869,
                    as_str(): "assert(check_prime(18) == false)",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 875,
                                        end: 881,
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
                                            src (ptr): 0x00007fe0cf4336a0,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 898,
                                                            end: 900,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 898,
                                                            end: 900,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 898,
                                                        end: 900,
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
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 882,
                                                                        end: 893,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                12,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 894,
                                                                            end: 896,
                                                                            as_str(): "12",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13386,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 882,
                                                                        end: 893,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 882,
                                                            end: 897,
                                                            as_str(): "check_prime(12)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3d29400,
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
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 901,
                                                                        end: 912,
                                                                        as_str(): "check_prime",
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
                                                                            as_str(): "n",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                18,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 913,
                                                                            end: 915,
                                                                            as_str(): "18",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13388,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 393,
                                                                    as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 901,
                                                                        end: 912,
                                                                        as_str(): "check_prime",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 901,
                                                            end: 916,
                                                            as_str(): "check_prime(18)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13389,
                                                Span {
                                                    src (ptr): 0x00007fe0d3d29400,
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
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 898,
                                                        end: 900,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 882,
                                            end: 916,
                                            as_str(): "check_prime(12) == check_prime(18)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13390,
                                Span {
                                    src (ptr): 0x00007fe0cf4336a0,
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 875,
                                        end: 881,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31771,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 875,
                            end: 917,
                            as_str(): "assert(check_prime(12) == check_prime(18))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 875,
                    end: 917,
                    as_str(): "assert(check_prime(12) == check_prime(18))",
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
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 924,
                            end: 928,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd209f60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                    ),
                    start: 924,
                    end: 928,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fd209f60,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
        ),
        start: 395,
        end: 930,
        as_str(): "fn main() -> bool {\n    assert(check_prime(64) == false);\n    assert(check_prime(8) == false);\n    assert(check_prime(7) == true);\n    assert(check_prime(11) == true);\n    assert(check_prime(13) == true);\n    assert(check_prime(2) == true);\n    assert(check_prime(3) == true);\n    assert(check_prime(1) == false);\n    assert(check_prime(0) == false);\n\n    assert(check_prime(11) == check_prime(17));\n    assert(check_prime(12) == false);\n    assert(check_prime(18) == false);\n    assert(check_prime(12) == check_prime(18));\n\n    true\n}",
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
        src (ptr): 0x00007fe0fd209f60,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
        ),
        start: 408,
        end: 412,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

