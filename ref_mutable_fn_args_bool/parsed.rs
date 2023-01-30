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
                            src (ptr): 0x00007fe06da36640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
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
                                                                    src (ptr): 0x00007fe06da36640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                    ),
                                                                    start: 43,
                                                                    end: 44,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06da36640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                            ),
                                                            start: 43,
                                                            end: 44,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Boolean(
                                                            true,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe06da36640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                        ),
                                                        start: 47,
                                                        end: 51,
                                                        as_str(): "true",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06da36640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                            ),
                                            start: 43,
                                            end: 51,
                                            as_str(): "b = true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06da36640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                    ),
                                    start: 43,
                                    end: 51,
                                    as_str(): "b = true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe06da36640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                            ),
                            start: 37,
                            end: 54,
                            as_str(): "{\n    b = true;\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06da36640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
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
                                src (ptr): 0x00007fe06da36640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                ),
                                start: 20,
                                end: 27,
                                as_str(): "ref mut",
                            },
                            type_info: Boolean,
                            type_span: Span {
                                src (ptr): 0x00007fe06da36640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                ),
                                start: 31,
                                end: 35,
                                as_str(): "bool",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe06da36640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                        ),
                        start: 9,
                        end: 54,
                        as_str(): "fn mut_arg(ref mut b: bool) {\n    b = true;\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe06da36640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                        ),
                        start: 9,
                        end: 36,
                        as_str(): "fn mut_arg(ref mut b: bool)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06da36640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
            ),
            start: 9,
            end: 54,
            as_str(): "fn mut_arg(ref mut b: bool) {\n    b = true;\n}",
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
                            src (ptr): 0x00007fe06da36640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                            ),
                            start: 59,
                            end: 63,
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
                                                    src (ptr): 0x00007fe06da36640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                    ),
                                                    start: 88,
                                                    end: 89,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06da36640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                    ),
                                                    start: 92,
                                                    end: 97,
                                                    as_str(): "false",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06da36640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                    ),
                                    start: 80,
                                    end: 98,
                                    as_str(): "let mut b = false;",
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
                                                                src (ptr): 0x00007fe06da36640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                ),
                                                                start: 103,
                                                                end: 110,
                                                                as_str(): "mut_arg",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06da36640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                        ),
                                                        start: 103,
                                                        end: 110,
                                                        as_str(): "mut_arg",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06da36640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                    ),
                                                                    start: 111,
                                                                    end: 112,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06da36640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                            ),
                                                            start: 111,
                                                            end: 112,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06da36640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                            ),
                                            start: 103,
                                            end: 113,
                                            as_str(): "mut_arg(b)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06da36640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                    ),
                                    start: 103,
                                    end: 113,
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
                                                    src (ptr): 0x00007fe06da36640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                    ),
                                                    start: 119,
                                                    end: 120,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06da36640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                            ),
                                            start: 119,
                                            end: 120,
                                            as_str(): "b",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06da36640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                    ),
                                    start: 119,
                                    end: 120,
                                    as_str(): "b",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe06da36640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                            ),
                            start: 74,
                            end: 122,
                            as_str(): "{\n    let mut b = false;\n    mut_arg(b);\n    b\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe06da36640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                        ),
                        start: 56,
                        end: 122,
                        as_str(): "fn main() -> bool {\n    let mut b = false;\n    mut_arg(b);\n    b\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe06da36640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                        ),
                        start: 69,
                        end: 73,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06da36640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
            ),
            start: 56,
            end: 122,
            as_str(): "fn main() -> bool {\n    let mut b = false;\n    mut_arg(b);\n    b\n}",
        },
    },
]
