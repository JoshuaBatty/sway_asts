TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe032b8a4f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
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
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
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
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 54,
                                        end: 58,
                                        as_str(): "2u64",
                                    },
                                },
                            ],
                        },
                        return_type: TypeId(
                            7263,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe032b8a4f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                            ),
                            start: 47,
                            end: 59,
                            as_str(): "(1u32, 2u64)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
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
        src (ptr): 0x00007fe032b8a4f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
        ),
        start: 9,
        end: 61,
        as_str(): "fn gimme_a_pair() -> (u32, u64) {\n    (1u32, 2u64)\n}",
    },
    attributes: {},
    return_type: TypeId(
        7261,
    ),
    initial_return_type: TypeId(
        7260,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe032b8a4f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
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
            src (ptr): 0x00007fe032b8a4f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
            ),
            start: 66,
            end: 70,
            as_str(): "test",
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
                                name_override_opt: Some(
                                    "__tuple_1",
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 95,
                                    end: 123,
                                    as_str(): "let (x, y): (T, E) = (a, b);",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Tuple {
                                    fields: [
                                        TyExpression {
                                            expression: VariableExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe032b8a4f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                        ),
                                                        start: 77,
                                                        end: 78,
                                                        as_str(): "a",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 117,
                                                    end: 118,
                                                    as_str(): "a",
                                                },
                                                mutability: Immutable,
                                            },
                                            return_type: TypeId(
                                                7265,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 117,
                                                end: 118,
                                                as_str(): "a",
                                            },
                                        },
                                        TyExpression {
                                            expression: VariableExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe032b8a4f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                        ),
                                                        start: 83,
                                                        end: 84,
                                                        as_str(): "b",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 120,
                                                    end: 121,
                                                    as_str(): "b",
                                                },
                                                mutability: Immutable,
                                            },
                                            return_type: TypeId(
                                                7266,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 120,
                                                end: 121,
                                                as_str(): "b",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7274,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 116,
                                    end: 122,
                                    as_str(): "(a, b)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7274,
                            ),
                            type_ascription: TypeId(
                                7272,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 107,
                                    end: 113,
                                    as_str(): "(T, E)",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 95,
                    end: 123,
                    as_str(): "let (x, y): (T, E) = (a, b);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 100,
                                    end: 101,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: TupleElemAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_1",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 95,
                                                    end: 123,
                                                    as_str(): "let (x, y): (T, E) = (a, b);",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 95,
                                                end: 123,
                                                as_str(): "let (x, y): (T, E) = (a, b);",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7277,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 95,
                                            end: 123,
                                            as_str(): "let (x, y): (T, E) = (a, b);",
                                        },
                                    },
                                    elem_to_access_num: 0,
                                    resolved_type_of_parent: TypeId(
                                        7277,
                                    ),
                                    elem_to_access_span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 123,
                                        as_str(): "let (x, y): (T, E) = (a, b);",
                                    },
                                },
                                return_type: TypeId(
                                    7265,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 95,
                                    end: 123,
                                    as_str(): "let (x, y): (T, E) = (a, b);",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7265,
                            ),
                            type_ascription: TypeId(
                                7265,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 108,
                                    end: 109,
                                    as_str(): "T",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 95,
                    end: 123,
                    as_str(): "let (x, y): (T, E) = (a, b);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 103,
                                    end: 104,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: TupleElemAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_1",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 95,
                                                    end: 123,
                                                    as_str(): "let (x, y): (T, E) = (a, b);",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 95,
                                                end: 123,
                                                as_str(): "let (x, y): (T, E) = (a, b);",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7280,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 95,
                                            end: 123,
                                            as_str(): "let (x, y): (T, E) = (a, b);",
                                        },
                                    },
                                    elem_to_access_num: 1,
                                    resolved_type_of_parent: TypeId(
                                        7280,
                                    ),
                                    elem_to_access_span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 123,
                                        as_str(): "let (x, y): (T, E) = (a, b);",
                                    },
                                },
                                return_type: TypeId(
                                    7266,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 95,
                                    end: 123,
                                    as_str(): "let (x, y): (T, E) = (a, b);",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7266,
                            ),
                            type_ascription: TypeId(
                                7266,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 111,
                                    end: 112,
                                    as_str(): "E",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 95,
                    end: 123,
                    as_str(): "let (x, y): (T, E) = (a, b);",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 77,
                    end: 78,
                    as_str(): "a",
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
                7265,
            ),
            initial_type_id: TypeId(
                7267,
            ),
            type_span: Span {
                src (ptr): 0x00007fe032b8a4f0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                ),
                start: 80,
                end: 81,
                as_str(): "T",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 83,
                    end: 84,
                    as_str(): "b",
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
                7266,
            ),
            initial_type_id: TypeId(
                7268,
            ),
            type_span: Span {
                src (ptr): 0x00007fe032b8a4f0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                ),
                start: 86,
                end: 87,
                as_str(): "E",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe032b8a4f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
        ),
        start: 63,
        end: 125,
        as_str(): "fn test<T, E>(a: T, b: E) {\n    let (x, y): (T, E) = (a, b);\n}",
    },
    attributes: {},
    return_type: TypeId(
        7270,
    ),
    initial_return_type: TypeId(
        7269,
    ),
    type_parameters: [
        T: TypeId(7265),
        E: TypeId(7266),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe032b8a4f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
        ),
        start: 63,
        end: 88,
        as_str(): "fn test<T, E>(a: T, b: E)",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe032b8a4f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
            ),
            start: 131,
            end: 135,
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
                                name_override_opt: Some(
                                    "__tuple_2",
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 151,
                                    end: 183,
                                    as_str(): "let (foo, bar) = gimme_a_pair();",
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
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 168,
                                                end: 180,
                                                as_str(): "gimme_a_pair",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        547,
                                        Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
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
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 168,
                                                end: 180,
                                                as_str(): "gimme_a_pair",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7284,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 168,
                                    end: 182,
                                    as_str(): "gimme_a_pair()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7284,
                            ),
                            type_ascription: TypeId(
                                7283,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 151,
                    end: 183,
                    as_str(): "let (foo, bar) = gimme_a_pair();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 156,
                                    end: 159,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: TupleElemAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_2",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 151,
                                                    end: 183,
                                                    as_str(): "let (foo, bar) = gimme_a_pair();",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 151,
                                                end: 183,
                                                as_str(): "let (foo, bar) = gimme_a_pair();",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7287,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 151,
                                            end: 183,
                                            as_str(): "let (foo, bar) = gimme_a_pair();",
                                        },
                                    },
                                    elem_to_access_num: 0,
                                    resolved_type_of_parent: TypeId(
                                        7287,
                                    ),
                                    elem_to_access_span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 151,
                                        end: 183,
                                        as_str(): "let (foo, bar) = gimme_a_pair();",
                                    },
                                },
                                return_type: TypeId(
                                    33,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 151,
                                    end: 183,
                                    as_str(): "let (foo, bar) = gimme_a_pair();",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7285,
                            ),
                            type_ascription: TypeId(
                                7285,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 151,
                    end: 183,
                    as_str(): "let (foo, bar) = gimme_a_pair();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 161,
                                    end: 164,
                                    as_str(): "bar",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: TupleElemAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_2",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 151,
                                                    end: 183,
                                                    as_str(): "let (foo, bar) = gimme_a_pair();",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 151,
                                                end: 183,
                                                as_str(): "let (foo, bar) = gimme_a_pair();",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7290,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 151,
                                            end: 183,
                                            as_str(): "let (foo, bar) = gimme_a_pair();",
                                        },
                                    },
                                    elem_to_access_num: 1,
                                    resolved_type_of_parent: TypeId(
                                        7290,
                                    ),
                                    elem_to_access_span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 151,
                                        end: 183,
                                        as_str(): "let (foo, bar) = gimme_a_pair();",
                                    },
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 151,
                                    end: 183,
                                    as_str(): "let (foo, bar) = gimme_a_pair();",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7288,
                            ),
                            type_ascription: TypeId(
                                7288,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 151,
                    end: 183,
                    as_str(): "let (foo, bar) = gimme_a_pair();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: Some(
                                    "__tuple_3",
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 188,
                                    end: 225,
                                    as_str(): "let (x, y): (u32, bool) = (10, true);",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Tuple {
                                    fields: [
                                        TyExpression {
                                            expression: Literal(
                                                U32(
                                                    10,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                7293,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 215,
                                                end: 217,
                                                as_str(): "10",
                                            },
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
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 219,
                                                end: 223,
                                                as_str(): "true",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7295,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 214,
                                    end: 224,
                                    as_str(): "(10, true)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7295,
                            ),
                            type_ascription: TypeId(
                                7292,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 200,
                                    end: 211,
                                    as_str(): "(u32, bool)",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 188,
                    end: 225,
                    as_str(): "let (x, y): (u32, bool) = (10, true);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 193,
                                    end: 194,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: TupleElemAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_3",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 188,
                                                    end: 225,
                                                    as_str(): "let (x, y): (u32, bool) = (10, true);",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 188,
                                                end: 225,
                                                as_str(): "let (x, y): (u32, bool) = (10, true);",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7297,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 188,
                                            end: 225,
                                            as_str(): "let (x, y): (u32, bool) = (10, true);",
                                        },
                                    },
                                    elem_to_access_num: 0,
                                    resolved_type_of_parent: TypeId(
                                        7297,
                                    ),
                                    elem_to_access_span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 188,
                                        end: 225,
                                        as_str(): "let (x, y): (u32, bool) = (10, true);",
                                    },
                                },
                                return_type: TypeId(
                                    7293,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 188,
                                    end: 225,
                                    as_str(): "let (x, y): (u32, bool) = (10, true);",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33,
                            ),
                            type_ascription: TypeId(
                                33,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 201,
                                    end: 204,
                                    as_str(): "u32",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 188,
                    end: 225,
                    as_str(): "let (x, y): (u32, bool) = (10, true);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 196,
                                    end: 197,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: TupleElemAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_3",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 188,
                                                    end: 225,
                                                    as_str(): "let (x, y): (u32, bool) = (10, true);",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 188,
                                                end: 225,
                                                as_str(): "let (x, y): (u32, bool) = (10, true);",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7299,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 188,
                                            end: 225,
                                            as_str(): "let (x, y): (u32, bool) = (10, true);",
                                        },
                                    },
                                    elem_to_access_num: 1,
                                    resolved_type_of_parent: TypeId(
                                        7299,
                                    ),
                                    elem_to_access_span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 188,
                                        end: 225,
                                        as_str(): "let (x, y): (u32, bool) = (10, true);",
                                    },
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 188,
                                    end: 225,
                                    as_str(): "let (x, y): (u32, bool) = (10, true);",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                71,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 206,
                                    end: 210,
                                    as_str(): "bool",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 188,
                    end: 225,
                    as_str(): "let (x, y): (u32, bool) = (10, true);",
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
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 305,
                                        end: 309,
                                        as_str(): "test",
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
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 77,
                                            end: 78,
                                            as_str(): "a",
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
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 310,
                                            end: 314,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 83,
                                            end: 84,
                                            as_str(): "b",
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
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 316,
                                            end: 321,
                                            as_str(): "false",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                549,
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 63,
                                    end: 125,
                                    as_str(): "fn test<T, E>(a: T, b: E) {\n    let (x, y): (T, E) = (a, b);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 305,
                                        end: 309,
                                        as_str(): "test",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7312,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe032b8a4f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                            ),
                            start: 305,
                            end: 322,
                            as_str(): "test(true, false)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 305,
                    end: 322,
                    as_str(): "test(true, false)",
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
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 328,
                                        end: 332,
                                        as_str(): "test",
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
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 77,
                                            end: 78,
                                            as_str(): "a",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Literal(
                                            U64(
                                                42,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 334,
                                            end: 336,
                                            as_str(): "42",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 83,
                                            end: 84,
                                            as_str(): "b",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Literal(
                                            U64(
                                                42,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 338,
                                            end: 340,
                                            as_str(): "42",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                551,
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 63,
                                    end: 125,
                                    as_str(): "fn test<T, E>(a: T, b: E) {\n    let (x, y): (T, E) = (a, b);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 328,
                                        end: 332,
                                        as_str(): "test",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7327,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe032b8a4f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                            ),
                            start: 328,
                            end: 341,
                            as_str(): "test (42, 42)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 328,
                    end: 341,
                    as_str(): "test (42, 42)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: Some(
                                    "__tuple_4",
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 347,
                                    end: 437,
                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Tuple {
                                    fields: [
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    42,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 405,
                                                end: 410,
                                                as_str(): "42u64",
                                            },
                                        },
                                        TyExpression {
                                            expression: Tuple {
                                                fields: [
                                                    TyExpression {
                                                        expression: Literal(
                                                            U32(
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 413,
                                                            end: 418,
                                                            as_str(): "42u32",
                                                        },
                                                    },
                                                    TyExpression {
                                                        expression: Tuple {
                                                            fields: [
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
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 421,
                                                                        end: 425,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        String(
                                                                            Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 428,
                                                                                end: 430,
                                                                                as_str(): "ok",
                                                                            },
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        7332,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 427,
                                                                        end: 431,
                                                                        as_str(): "\"ok\"",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        return_type: TypeId(
                                                            7334,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 420,
                                                            end: 432,
                                                            as_str(): "(true, \"ok\")",
                                                        },
                                                    },
                                                ],
                                            },
                                            return_type: TypeId(
                                                7337,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 412,
                                                end: 434,
                                                as_str(): "(42u32, (true, \"ok\") )",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7341,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 404,
                                    end: 436,
                                    as_str(): "(42u64, (42u32, (true, \"ok\") ) )",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7341,
                            ),
                            type_ascription: TypeId(
                                7331,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 371,
                                    end: 401,
                                    as_str(): "(u64, (u32, (bool, str[2]) ) )",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 347,
                    end: 437,
                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 352,
                                    end: 353,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: TupleElemAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_4",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 347,
                                                    end: 437,
                                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 347,
                                                end: 437,
                                                as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7345,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 347,
                                            end: 437,
                                            as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                        },
                                    },
                                    elem_to_access_num: 0,
                                    resolved_type_of_parent: TypeId(
                                        7345,
                                    ),
                                    elem_to_access_span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 347,
                                        end: 437,
                                        as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                    },
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 347,
                                    end: 437,
                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                21,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 372,
                                    end: 375,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 347,
                    end: 437,
                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: Some(
                                    "__tuple_5",
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 347,
                                    end: 437,
                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: TupleElemAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_4",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 347,
                                                    end: 437,
                                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 347,
                                                end: 437,
                                                as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7352,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 347,
                                            end: 437,
                                            as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                        },
                                    },
                                    elem_to_access_num: 1,
                                    resolved_type_of_parent: TypeId(
                                        7352,
                                    ),
                                    elem_to_access_span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 347,
                                        end: 437,
                                        as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                    },
                                },
                                return_type: TypeId(
                                    7354,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 347,
                                    end: 437,
                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7354,
                            ),
                            type_ascription: TypeId(
                                7348,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 377,
                                    end: 399,
                                    as_str(): "(u32, (bool, str[2]) )",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 347,
                    end: 437,
                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 356,
                                    end: 357,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: TupleElemAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_5",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 347,
                                                    end: 437,
                                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 347,
                                                end: 437,
                                                as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7357,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 347,
                                            end: 437,
                                            as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                        },
                                    },
                                    elem_to_access_num: 0,
                                    resolved_type_of_parent: TypeId(
                                        7357,
                                    ),
                                    elem_to_access_span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 347,
                                        end: 437,
                                        as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                    },
                                },
                                return_type: TypeId(
                                    33,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 347,
                                    end: 437,
                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33,
                            ),
                            type_ascription: TypeId(
                                33,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 378,
                                    end: 381,
                                    as_str(): "u32",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 347,
                    end: 437,
                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: Some(
                                    "__tuple_6",
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 347,
                                    end: 437,
                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: TupleElemAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_5",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 347,
                                                    end: 437,
                                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 347,
                                                end: 437,
                                                as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7362,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 347,
                                            end: 437,
                                            as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                        },
                                    },
                                    elem_to_access_num: 1,
                                    resolved_type_of_parent: TypeId(
                                        7362,
                                    ),
                                    elem_to_access_span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 347,
                                        end: 437,
                                        as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                    },
                                },
                                return_type: TypeId(
                                    7363,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 347,
                                    end: 437,
                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7363,
                            ),
                            type_ascription: TypeId(
                                7359,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 383,
                                    end: 397,
                                    as_str(): "(bool, str[2])",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 347,
                    end: 437,
                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 360,
                                    end: 361,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: TupleElemAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_6",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 347,
                                                    end: 437,
                                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 347,
                                                end: 437,
                                                as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7365,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 347,
                                            end: 437,
                                            as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                        },
                                    },
                                    elem_to_access_num: 0,
                                    resolved_type_of_parent: TypeId(
                                        7365,
                                    ),
                                    elem_to_access_span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 347,
                                        end: 437,
                                        as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                    },
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 347,
                                    end: 437,
                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                71,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 384,
                                    end: 388,
                                    as_str(): "bool",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 347,
                    end: 437,
                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 363,
                                    end: 364,
                                    as_str(): "d",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: TupleElemAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_6",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 347,
                                                    end: 437,
                                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe032b8a4f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                ),
                                                start: 347,
                                                end: 437,
                                                as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7367,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 347,
                                            end: 437,
                                            as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                        },
                                    },
                                    elem_to_access_num: 1,
                                    resolved_type_of_parent: TypeId(
                                        7367,
                                    ),
                                    elem_to_access_span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 347,
                                        end: 437,
                                        as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                    },
                                },
                                return_type: TypeId(
                                    7332,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 347,
                                    end: 437,
                                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7332,
                            ),
                            type_ascription: TypeId(
                                7253,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 390,
                                    end: 396,
                                    as_str(): "str[2]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 347,
                    end: 437,
                    as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 352,
                                    end: 353,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe032b8a4f0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                ),
                                start: 442,
                                end: 443,
                                as_str(): "a",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe032b8a4f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                            ),
                            start: 442,
                            end: 443,
                            as_str(): "a",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe032b8a4f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                    ),
                    start: 442,
                    end: 443,
                    as_str(): "a",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe032b8a4f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
        ),
        start: 128,
        end: 445,
        as_str(): "fn main() -> u32 {\n    let (foo, bar) = gimme_a_pair();\n    let (x, y): (u32, bool) = (10, true);\n    //let (x, y): (u32, _) = (42, true); // this generates a parsing error\n    test(true, false);\n    test (42, 42);\n    let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );\n    a\n}",
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
        src (ptr): 0x00007fe032b8a4f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
        ),
        start: 141,
        end: 144,
        as_str(): "u32",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

