[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb126e2ab50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                            ),
                            start: 13,
                            end: 17,
                            as_str(): "core",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb126e2ab50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                            ),
                            start: 19,
                            end: 22,
                            as_str(): "ops",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Star,
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb126e2ab50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
            ),
            start: 9,
            end: 26,
            as_str(): "use core::ops::*;",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        1,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        2,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: AmbiguousPathExpression(
                                            AmbiguousPathExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: AmbiguousSuffix {
                                                            before: TypeBinding {
                                                                inner: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb126e2ab50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                        ),
                                                                        start: 87,
                                                                        end: 90,
                                                                        as_str(): "u64",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb126e2ab50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                    ),
                                                                    start: 87,
                                                                    end: 90,
                                                                    as_str(): "u64",
                                                                },
                                                            },
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
                                                        },
                                                        is_absolute: false,
                                                    },
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
                                                args: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb126e2ab50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                    ),
                                                                    start: 103,
                                                                    end: 104,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
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
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb126e2ab50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                    ),
                                                                    start: 106,
                                                                    end: 107,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
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
                                                ],
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fb126e2ab50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                            ),
                            start: 45,
                            end: 110,
                            as_str(): "{\n    let a = 1u64;\n    let b = 2u64;\n    u64::binary_xor(a, b)\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb126e2ab50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                        ),
                        start: 28,
                        end: 110,
                        as_str(): "fn main() -> u64 {\n    let a = 1u64;\n    let b = 2u64;\n    u64::binary_xor(a, b)\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb126e2ab50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
            ),
            start: 28,
            end: 110,
            as_str(): "fn main() -> u64 {\n    let a = 1u64;\n    let b = 2u64;\n    u64::binary_xor(a, b)\n}",
        },
    },
]
