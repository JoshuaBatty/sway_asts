
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0ceb55870,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
            ),
            start: 38,
            end: 39,
            as_str(): "f",
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
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 44,
                                            as_str(): "cond",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0ceb55870,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                        ),
                                        start: 68,
                                        end: 72,
                                        as_str(): "cond",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ceb55870,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                    ),
                                    start: 68,
                                    end: 72,
                                    as_str(): "cond",
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
                                                            U64(
                                                                10,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 83,
                                                            end: 85,
                                                            as_str(): "10",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0ceb55870,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                    ),
                                                    start: 83,
                                                    end: 85,
                                                    as_str(): "10",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ceb55870,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                    ),
                                    start: 73,
                                    end: 91,
                                    as_str(): "{\n        10\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
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
                                                                src (ptr): 0x00007fe0ceb55870,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                ),
                                                                start: 107,
                                                                end: 109,
                                                                as_str(): "20",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ceb55870,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                        ),
                                                        start: 107,
                                                        end: 109,
                                                        as_str(): "20",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0ceb55870,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                        ),
                                        start: 97,
                                        end: 115,
                                        as_str(): "{\n        20\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0ceb55870,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                            ),
                            start: 65,
                            end: 115,
                            as_str(): "if cond {\n        10\n    } else {\n        20\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0ceb55870,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                    ),
                    start: 65,
                    end: 115,
                    as_str(): "if cond {\n        10\n    } else {\n        20\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0ceb55870,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                    ),
                    start: 40,
                    end: 44,
                    as_str(): "cond",
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
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0ceb55870,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                ),
                start: 46,
                end: 50,
                as_str(): "bool",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0ceb55870,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
        ),
        start: 35,
        end: 117,
        as_str(): "fn f(cond: bool) -> u64 {\n    if cond {\n        10\n    } else {\n        20\n    }\n}",
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
        src (ptr): 0x00007fe0ceb55870,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
        ),
        start: 55,
        end: 58,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0ceb55870,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
            ),
            start: 122,
            end: 126,
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
                                        src (ptr): 0x00007fe0ceb55870,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                        ),
                                        start: 135,
                                        end: 136,
                                        as_str(): "f",
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
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 44,
                                            as_str(): "cond",
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
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 137,
                                            end: 141,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13316,
                                Span {
                                    src (ptr): 0x00007fe0ceb55870,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                    ),
                                    start: 35,
                                    end: 117,
                                    as_str(): "fn f(cond: bool) -> u64 {\n    if cond {\n        10\n    } else {\n        20\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0ceb55870,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                        ),
                                        start: 135,
                                        end: 136,
                                        as_str(): "f",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0ceb55870,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                            ),
                            start: 135,
                            end: 142,
                            as_str(): "f(true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0ceb55870,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                    ),
                    start: 135,
                    end: 142,
                    as_str(): "f(true)",
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
                                        src (ptr): 0x00007fe0ceb55870,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                        ),
                                        start: 148,
                                        end: 154,
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
                                            src (ptr): 0x00007fe0df4310f0,
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
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 164,
                                                            end: 166,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 164,
                                                            end: 166,
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
                                                        src (ptr): 0x00007fe0ceb55870,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                        ),
                                                        start: 164,
                                                        end: 166,
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
                                                            src (ptr): 0x00007fe0df3dd640,
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
                                                                        src (ptr): 0x00007fe0ceb55870,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                        ),
                                                                        start: 155,
                                                                        end: 156,
                                                                        as_str(): "f",
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
                                                                            src (ptr): 0x00007fe0ceb55870,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                            ),
                                                                            start: 40,
                                                                            end: 44,
                                                                            as_str(): "cond",
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
                                                                            src (ptr): 0x00007fe0ceb55870,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                            ),
                                                                            start: 157,
                                                                            end: 162,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13319,
                                                                Span {
                                                                    src (ptr): 0x00007fe0ceb55870,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                    ),
                                                                    start: 35,
                                                                    end: 117,
                                                                    as_str(): "fn f(cond: bool) -> u64 {\n    if cond {\n        10\n    } else {\n        20\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ceb55870,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                        ),
                                                                        start: 155,
                                                                        end: 156,
                                                                        as_str(): "f",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 155,
                                                            end: 163,
                                                            as_str(): "f(false)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0df3dd640,
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
                                                                20,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 167,
                                                            end: 169,
                                                            as_str(): "20",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13320,
                                                Span {
                                                    src (ptr): 0x00007fe0df3dd640,
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
                                                        src (ptr): 0x00007fe0ceb55870,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                        ),
                                                        start: 164,
                                                        end: 166,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 155,
                                            end: 169,
                                            as_str(): "f(false) == 20",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13321,
                                Span {
                                    src (ptr): 0x00007fe0df4310f0,
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
                                        src (ptr): 0x00007fe0ceb55870,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                        ),
                                        start: 148,
                                        end: 154,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31646,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0ceb55870,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                            ),
                            start: 148,
                            end: 170,
                            as_str(): "assert(f(false) == 20)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0ceb55870,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                    ),
                    start: 148,
                    end: 170,
                    as_str(): "assert(f(false) == 20)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0ceb55870,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
        ),
        start: 119,
        end: 173,
        as_str(): "fn main() {\n    f(true);\n    assert(f(false) == 20);\n}",
    },
    attributes: {},
    return_type: TypeId(
        31637,
    ),
    initial_return_type: TypeId(
        31636,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0ceb55870,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
        ),
        start: 119,
        end: 128,
        as_str(): "fn main()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

