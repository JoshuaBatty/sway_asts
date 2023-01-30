[
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe06de68e50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                            ),
                            start: 12,
                            end: 19,
                            as_str(): "mut_arg",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Reassignment(
                                            ReassignmentExpression {
                                                lhs: VariableExpression(
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06de68e50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                    ),
                                                                    start: 42,
                                                                    end: 43,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06de68e50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                            ),
                                                            start: 42,
                                                            end: 43,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            20,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe06de68e50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                        ),
                                                        start: 46,
                                                        end: 48,
                                                        as_str(): "20",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06de68e50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                            ),
                                            start: 42,
                                            end: 48,
                                            as_str(): "b = 20",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06de68e50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                    ),
                                    start: 42,
                                    end: 48,
                                    as_str(): "b = 20",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe06de68e50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                            ),
                            start: 36,
                            end: 51,
                            as_str(): "{\n    b = 20;\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06de68e50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                    ),
                                    start: 28,
                                    end: 29,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: true,
                            is_mutable: true,
                            mutability_span: Span {
                                src (ptr): 0x00007fe06de68e50,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                ),
                                start: 20,
                                end: 27,
                                as_str(): "ref mut",
                            },
                            type_info: UnsignedInteger(
                                ThirtyTwo,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe06de68e50,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                ),
                                start: 31,
                                end: 34,
                                as_str(): "u32",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe06de68e50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                        ),
                        start: 9,
                        end: 51,
                        as_str(): "fn mut_arg(ref mut b: u32) {\n    b = 20;\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe06de68e50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                        ),
                        start: 9,
                        end: 35,
                        as_str(): "fn mut_arg(ref mut b: u32)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06de68e50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
            ),
            start: 9,
            end: 51,
            as_str(): "fn mut_arg(ref mut b: u32) {\n    b = 20;\n}",
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
                            src (ptr): 0x00007fe06de68e50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                            ),
                            start: 56,
                            end: 60,
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
                                                    src (ptr): 0x00007fe06de68e50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                    ),
                                                    start: 84,
                                                    end: 85,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U32(
                                                        0,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06de68e50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                    ),
                                                    start: 88,
                                                    end: 92,
                                                    as_str(): "0u32",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06de68e50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                    ),
                                    start: 76,
                                    end: 93,
                                    as_str(): "let mut b = 0u32;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06de68e50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                ),
                                                                start: 98,
                                                                end: 105,
                                                                as_str(): "mut_arg",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06de68e50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                        ),
                                                        start: 98,
                                                        end: 105,
                                                        as_str(): "mut_arg",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06de68e50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                    ),
                                                                    start: 106,
                                                                    end: 107,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06de68e50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
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
                                            src (ptr): 0x00007fe06de68e50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                            ),
                                            start: 98,
                                            end: 108,
                                            as_str(): "mut_arg(b)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06de68e50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                    ),
                                    start: 98,
                                    end: 108,
                                    as_str(): "mut_arg(b)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe06de68e50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                    ),
                                                    start: 114,
                                                    end: 115,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06de68e50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                            ),
                                            start: 114,
                                            end: 115,
                                            as_str(): "b",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06de68e50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                    ),
                                    start: 114,
                                    end: 115,
                                    as_str(): "b",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe06de68e50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                            ),
                            start: 70,
                            end: 117,
                            as_str(): "{\n    let mut b = 0u32;\n    mut_arg(b);\n    b\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe06de68e50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                        ),
                        start: 53,
                        end: 117,
                        as_str(): "fn main() -> u32 {\n    let mut b = 0u32;\n    mut_arg(b);\n    b\n}",
                    },
                    return_type: UnsignedInteger(
                        ThirtyTwo,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe06de68e50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                        ),
                        start: 66,
                        end: 69,
                        as_str(): "u32",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06de68e50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
            ),
            start: 53,
            end: 117,
            as_str(): "fn main() -> u32 {\n    let mut b = 0u32;\n    mut_arg(b);\n    b\n}",
        },
    },
]
