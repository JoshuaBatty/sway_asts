
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb126e2ab50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
            ),
            start: 31,
            end: 35,
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
                                    src (ptr): 0x00007fb126e2ab50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                    ),
                                    start: 55,
                                    end: 56,
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
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb126e2ab50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                    ),
                                    start: 59,
                                    end: 63,
                                    as_str(): "1u64",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7252,
                            ),
                            type_ascription: TypeId(
                                7252,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb126e2ab50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                    ),
                    start: 51,
                    end: 64,
                    as_str(): "let a = 1u64;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb126e2ab50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                    ),
                                    start: 73,
                                    end: 74,
                                    as_str(): "b",
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
                                    src (ptr): 0x00007fb126e2ab50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                    ),
                                    start: 77,
                                    end: 81,
                                    as_str(): "2u64",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7253,
                            ),
                            type_ascription: TypeId(
                                7253,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb126e2ab50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                    ),
                    start: 69,
                    end: 82,
                    as_str(): "let b = 2u64;",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb126e2ab50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                        ),
                                        start: 92,
                                        end: 102,
                                        as_str(): "binary_xor",
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
                                            src (ptr): 0x00007fb13303d8a0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 7928,
                                            end: 7932,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb126e2ab50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                    ),
                                                    start: 55,
                                                    end: 56,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb126e2ab50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                ),
                                                start: 103,
                                                end: 104,
                                                as_str(): "a",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7252,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb126e2ab50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                            ),
                                            start: 103,
                                            end: 104,
                                            as_str(): "a",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13303d8a0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 7934,
                                            end: 7939,
                                            as_str(): "other",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb126e2ab50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                    ),
                                                    start: 73,
                                                    end: 74,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb126e2ab50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                ),
                                                start: 106,
                                                end: 107,
                                                as_str(): "b",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7253,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb126e2ab50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                            ),
                                            start: 106,
                                            end: 107,
                                            as_str(): "b",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                544,
                                Span {
                                    src (ptr): 0x00007fb13303d8a0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                    ),
                                    start: 7914,
                                    end: 8057,
                                    as_str(): "fn binary_xor(self, other: Self) -> Self {\n        asm(r1: self, r2: other, r3) {\n            xor r3 r1 r2;\n            r3: u64\n        }\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb126e2ab50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                        ),
                                        start: 87,
                                        end: 102,
                                        as_str(): "u64::binary_xor",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb126e2ab50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                            ),
                            start: 87,
                            end: 108,
                            as_str(): "u64::binary_xor(a, b)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb126e2ab50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                    ),
                    start: 87,
                    end: 108,
                    as_str(): "u64::binary_xor(a, b)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb126e2ab50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
        ),
        start: 28,
        end: 110,
        as_str(): "fn main() -> u64 {\n    let a = 1u64;\n    let b = 2u64;\n    u64::binary_xor(a, b)\n}",
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
        src (ptr): 0x00007fb126e2ab50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
        ),
        start: 41,
        end: 44,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

