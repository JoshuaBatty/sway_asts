TyFunctionDeclaration {
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
                                lhs_type: TypeId(
                                    3,
                                ),
                                lhs_indices: [],
                                rhs: TyExpression {
                                    expression: Literal(
                                        Boolean(
                                            true,
                                        ),
                                    ),
                                    return_type: TypeId(
                                        3,
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
                        return_type: TypeId(
                            9,
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
    },
    parameters: [
        TyFunctionParameter {
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
            type_id: TypeId(
                3,
            ),
            initial_type_id: TypeId(
                3,
            ),
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
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe06da36640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
        ),
        start: 9,
        end: 54,
        as_str(): "fn mut_arg(ref mut b: bool) {\n    b = true;\n}",
    },
    attributes: {},
    return_type: TypeId(
        5,
    ),
    initial_return_type: TypeId(
        4,
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        false,
                                    ),
                                ),
                                return_type: TypeId(
                                    3,
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
                            mutability: Mutable,
                            return_type: TypeId(
                                3,
                            ),
                            type_ascription: TypeId(
                                12,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
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
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
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
                                    TyExpression {
                                        expression: VariableExpression {
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
                                            span: Span {
                                                src (ptr): 0x00007fe06da36640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                ),
                                                start: 111,
                                                end: 112,
                                                as_str(): "b",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            3,
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
                                ),
                            ],
                            function_decl_id: DeclId(
                                2,
                                Span {
                                    src (ptr): 0x00007fe06da36640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                    ),
                                    start: 9,
                                    end: 54,
                                    as_str(): "fn mut_arg(ref mut b: bool) {\n    b = true;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
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
                            ),
                        },
                        return_type: TypeId(
                            15,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
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
                            span: Span {
                                src (ptr): 0x00007fe06da36640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                ),
                                start: 119,
                                end: 120,
                                as_str(): "b",
                            },
                            mutability: Mutable,
                        },
                        return_type: TypeId(
                            3,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe06da36640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
        ),
        start: 56,
        end: 122,
        as_str(): "fn main() -> bool {\n    let mut b = false;\n    mut_arg(b);\n    b\n}",
    },
    attributes: {},
    return_type: TypeId(
        3,
    ),
    initial_return_type: TypeId(
        3,
    ),
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

