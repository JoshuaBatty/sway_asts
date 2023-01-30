TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a20305dd0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
            ),
            start: 12,
            end: 24,
            as_str(): "gimme_a_pair",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Tuple {
                            fields: [
                                TyExpression {
                                    expression: Literal(
                                        U32(
                                            1,
                                        ),
                                    ),
                                    return_type: TypeId(
                                        33,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007f8a20305dd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                        ),
                                        start: 48,
                                        end: 52,
                                        as_str(): "1u32",
                                    },
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
                                        src (ptr): 0x00007f8a20305dd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                        ),
                                        start: 54,
                                        end: 58,
                                        as_str(): "2u64",
                                    },
                                },
                            ],
                        },
                        return_type: TypeId(
                            7255,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a20305dd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                            ),
                            start: 47,
                            end: 59,
                            as_str(): "(1u32, 2u64)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a20305dd0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                    ),
                    start: 47,
                    end: 59,
                    as_str(): "(1u32, 2u64)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a20305dd0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
        ),
        start: 9,
        end: 61,
        as_str(): "fn gimme_a_pair() -> (u32, u64) {\n    (1u32, 2u64)\n}",
    },
    attributes: {},
    return_type: TypeId(
        7253,
    ),
    initial_return_type: TypeId(
        7252,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007f8a20305dd0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
        ),
        start: 30,
        end: 40,
        as_str(): "(u32, u64)",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a20305dd0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
            ),
            start: 66,
            end: 70,
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
                                    src (ptr): 0x00007f8a20305dd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                    ),
                                    start: 90,
                                    end: 91,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007f8a20305dd0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                ),
                                                start: 94,
                                                end: 106,
                                                as_str(): "gimme_a_pair",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        546,
                                        Span {
                                            src (ptr): 0x00007f8a20305dd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 61,
                                            as_str(): "fn gimme_a_pair() -> (u32, u64) {\n    (1u32, 2u64)\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007f8a20305dd0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                ),
                                                start: 94,
                                                end: 106,
                                                as_str(): "gimme_a_pair",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7258,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20305dd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                    ),
                                    start: 94,
                                    end: 108,
                                    as_str(): "gimme_a_pair()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7258,
                            ),
                            type_ascription: TypeId(
                                7257,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a20305dd0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                    ),
                    start: 86,
                    end: 109,
                    as_str(): "let x = gimme_a_pair();",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: TupleElemAccess {
                            prefix: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a20305dd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                            ),
                                            start: 90,
                                            end: 91,
                                            as_str(): "x",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a20305dd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                        ),
                                        start: 114,
                                        end: 115,
                                        as_str(): "x",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    7260,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20305dd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                    ),
                                    start: 114,
                                    end: 115,
                                    as_str(): "x",
                                },
                            },
                            elem_to_access_num: 0,
                            resolved_type_of_parent: TypeId(
                                7260,
                            ),
                            elem_to_access_span: Span {
                                src (ptr): 0x00007f8a20305dd0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                ),
                                start: 116,
                                end: 117,
                                as_str(): "0",
                            },
                        },
                        return_type: TypeId(
                            33,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a20305dd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                            ),
                            start: 114,
                            end: 117,
                            as_str(): "x.0",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a20305dd0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                    ),
                    start: 114,
                    end: 117,
                    as_str(): "x.0",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a20305dd0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
        ),
        start: 63,
        end: 119,
        as_str(): "fn main() -> u32 {\n    let x = gimme_a_pair();\n    x.0\n}",
    },
    attributes: {},
    return_type: TypeId(
        33,
    ),
    initial_return_type: TypeId(
        33,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007f8a20305dd0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
        ),
        start: 76,
        end: 79,
        as_str(): "u32",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

