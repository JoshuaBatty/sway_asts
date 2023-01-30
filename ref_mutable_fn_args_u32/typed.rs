TyFunctionDeclaration {
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
                                lhs_type: TypeId(
                                    3,
                                ),
                                lhs_indices: [],
                                rhs: TyExpression {
                                    expression: Literal(
                                        U32(
                                            20,
                                        ),
                                    ),
                                    return_type: TypeId(
                                        8,
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
                        return_type: TypeId(
                            10,
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
    },
    parameters: [
        TyFunctionParameter {
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
            type_id: TypeId(
                3,
            ),
            initial_type_id: TypeId(
                3,
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
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe06de68e50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
        ),
        start: 9,
        end: 51,
        as_str(): "fn mut_arg(ref mut b: u32) {\n    b = 20;\n}",
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
        src (ptr): 0x00007fe06de68e50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
        ),
        start: 9,
        end: 35,
        as_str(): "fn mut_arg(ref mut b: u32)",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: Literal(
                                    U32(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    3,
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
                            mutability: Mutable,
                            return_type: TypeId(
                                13,
                            ),
                            type_ascription: TypeId(
                                13,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
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
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
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
                                    TyExpression {
                                        expression: VariableExpression {
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
                                            span: Span {
                                                src (ptr): 0x00007fe06de68e50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                ),
                                                start: 106,
                                                end: 107,
                                                as_str(): "b",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            13,
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
                                ),
                            ],
                            function_decl_id: DeclId(
                                2,
                                Span {
                                    src (ptr): 0x00007fe06de68e50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                    ),
                                    start: 9,
                                    end: 51,
                                    as_str(): "fn mut_arg(ref mut b: u32) {\n    b = 20;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
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
                            ),
                        },
                        return_type: TypeId(
                            16,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
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
                            span: Span {
                                src (ptr): 0x00007fe06de68e50,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                ),
                                start: 114,
                                end: 115,
                                as_str(): "b",
                            },
                            mutability: Mutable,
                        },
                        return_type: TypeId(
                            13,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe06de68e50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
        ),
        start: 53,
        end: 117,
        as_str(): "fn main() -> u32 {\n    let mut b = 0u32;\n    mut_arg(b);\n    b\n}",
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
        src (ptr): 0x00007fe06de68e50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
        ),
        start: 66,
        end: 69,
        as_str(): "u32",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

